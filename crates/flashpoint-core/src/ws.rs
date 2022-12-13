pub struct WebsocketRegister<RecType, ResType> {
  pub cls: Box<dyn Fn(RecType) -> ResType + Send>,
}

type WebsocketPing = String;
type WebsocketPong = String;

pub struct WebsocketRegisters {
  pub ping: WebsocketRegister<WebsocketPing, WebsocketPong>,
}
