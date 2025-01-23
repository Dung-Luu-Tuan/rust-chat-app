use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use lazy_static::lazy_static;

// Biến toàn cục để lưu trữ các kết nối WebSocket cho mỗi phòng
lazy_static! {
    static ref ROOMS: Arc<Mutex<HashMap<Uuid, Vec<Addr<ChatSession>>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Hàm khởi tạo WebSocket
pub async fn ws_index(req: HttpRequest, stream: web::Payload, path: web::Path<Uuid>) -> HttpResponse {
    let room_id = path.into_inner();
    println!("New WebSocket connection for room: {}", room_id);

    match ws::start(ChatSession::new(room_id), &req, stream) {
        Ok(res) => res,
        Err(e) => {
            println!("WebSocket handshake failed: {:?}", e);
            HttpResponse::InternalServerError().body("WebSocket handshake failed")
        }
    }
}

// Struct đại diện cho một WebSocket session
pub struct ChatSession {
    room_id: Uuid,
}

impl ChatSession {
    pub fn new(room_id: Uuid) -> Self {
        ChatSession { room_id }
    }

    pub fn log(&self, message: &str) {
        println!("[Room: {}] {}", self.room_id, message);
    }

    // Phương thức để gửi tin nhắn đến tất cả các kết nối trong phòng
    pub fn broadcast_message(&self, message: &str) {
        // Khóa Mutex để truy cập vào danh sách kết nối phòng
        if let Some(room) = ROOMS.lock().unwrap().get_mut(&self.room_id) {
            for client in room.iter() {
                // Gửi message cho tất cả các kết nối bằng BroadcastText
                client.do_send(BroadcastText(message.to_string()));
            }
        }
    }
}

// Custom message để gửi tin nhắn văn bản
pub struct BroadcastText(pub String);

// Implement the Handler trait for ChatSession to handle BroadcastText messages
impl Message for BroadcastText {
    type Result = ();
}

impl Handler<BroadcastText> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastText, ctx: &mut Self::Context) {
        // Gửi tin nhắn văn bản tới WebSocket client
        ctx.text(msg.0); // msg.0 là tin nhắn chứa trong BroadcastText
    }
}

// Implement Actor cho ChatSession
impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.log("WebSocket connection started");

        // Lưu kết nối WebSocket vào phòng khi có người tham gia
        ROOMS.lock().unwrap()
            .entry(self.room_id)
            .or_insert_with(Vec::new)
            .push(ctx.address());

        // Gửi ping định kỳ
        ctx.run_interval(std::time::Duration::from_secs(10), |act, ctx| {
            act.log("Sending periodic ping");
            ctx.ping(b"Ping from server");
        });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.log("WebSocket connection stopped");

        // Khi kết nối bị đóng, xóa kết nối khỏi phòng
        ROOMS.lock().unwrap()
            .entry(self.room_id)
            .and_modify(|clients| {
                // Dùng ctx.address() để lấy địa chỉ của client hiện tại
                clients.retain(|client| client != &ctx.address());
            });
    }
}


// Implement StreamHandler để xử lý tin nhắn WebSocket
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                self.log(&format!("Received text message: {}", text));
                // Khi nhận tin nhắn, gửi lại cho tất cả các kết nối trong phòng
                self.broadcast_message(&text);
            }
            Ok(ws::Message::Binary(bin)) => {
                self.log("Received binary message");
                ctx.binary(bin);
            }
            Ok(ws::Message::Ping(ping)) => {
                self.log("Ping received, sending Pong");
                ctx.pong(&ping);
            }
            Ok(ws::Message::Pong(_)) => {
                self.log("Pong received");
            }
            Ok(ws::Message::Close(reason)) => {
                self.log(&format!("WebSocket closing: {:?}", reason));
            }
            Ok(ws::Message::Continuation(_)) => {
                self.log("Received continuation frame");
            }
            Ok(ws::Message::Nop) => {
                self.log("Received NOP message");
            }
            Err(e) => {
                self.log(&format!("WebSocket error: {:?}", e));
                ctx.close(Some(ws::CloseReason::from((ws::CloseCode::Error, "Protocol error"))));
            }
        }
    }
}
