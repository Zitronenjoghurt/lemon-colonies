use egui_macroquad::macroquad::time::get_time;
use lemon_colonies_core::math::rect::Rect;
use lemon_colonies_core::messages::client::ClientMessage;
use lemon_colonies_core::messages::server::ServerMessage;
use quad_net::web_socket::WebSocket;

#[derive(Default)]
pub struct Ws {
    state: WsState,
    incoming: Vec<ServerMessage>,
}

impl Ws {
    pub fn connect(&mut self) {
        if matches!(self.state, WsState::Idle | WsState::Error(_)) {
            match WebSocket::connect("/ws") {
                Ok(ws) => {
                    self.state = WsState::Connecting((ws, get_time()));
                }
                Err(err) => self.state = WsState::Error(format!("{err:?}")),
            }
        }
    }

    pub fn disconnect(&mut self, reason: impl AsRef<str>) {
        self.state = WsState::Error(reason.as_ref().to_string());
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, WsState::Connected(_))
    }

    pub fn update(&mut self, toasts: &mut egui_notify::Toasts) {
        if let WsState::Connecting((socket, start_time)) = &self.state {
            if socket.connected() {
                let old_state = std::mem::take(&mut self.state);
                if let WsState::Connecting((socket, _)) = old_state {
                    self.state = WsState::Connected(socket);
                }
            } else if get_time() - *start_time > 5.0 {
                self.state = WsState::Error("Connection timed out.".to_string());
                toasts.error("Connection timed out.");
            }
        }

        if let WsState::Connected(socket) = &mut self.state {
            if !socket.connected() {
                self.state = WsState::Error("Lost connection to server.".to_string());
                toasts.error("Disconnected from server.");
                return;
            }

            while let Some(bytes) = socket.try_recv() {
                match ServerMessage::from_bytes(&bytes) {
                    Ok(message) => self.incoming.push(message),
                    Err(err) => {
                        toasts.error(err.to_string());
                    }
                }
            }
        }
    }

    pub fn state(&self) -> &WsState {
        &self.state
    }

    pub fn drain_incoming(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.incoming)
    }
}

// Message send helpers
impl Ws {
    pub fn send_bytes(&mut self, bytes: &[u8]) {
        if let WsState::Connected(socket) = &mut self.state {
            socket.send_bytes(bytes)
        }
    }

    pub fn hello(&mut self) {
        self.send_bytes(&ClientMessage::Hello.as_bytes());
    }

    pub fn subscribe_chunks(&mut self, view_rect: Rect<i32>) {
        self.send_bytes(&ClientMessage::SubscribeToChunks(view_rect).as_bytes());
    }

    pub fn request_colony_positions(&mut self) {
        self.send_bytes(&ClientMessage::ColonyPositions.as_bytes());
    }
}

#[derive(Default)]
pub enum WsState {
    #[default]
    Idle,
    Connecting((WebSocket, f64)),
    Connected(WebSocket),
    Error(String),
}
