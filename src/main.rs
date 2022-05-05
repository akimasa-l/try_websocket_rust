// copied from https://qiita.com/nisei275/items/2c5c6d934bdae5d138d1

extern crate env_logger;
extern crate ws;

use chrono::{DateTime, Local};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

fn main() {
    // ロガーの初期化
    env_logger::init();

    // WebSocketの開始
    listen("127.0.0.1:3012", |out| Server {
        out: out,
        user_name: String::new(),
    })
    .unwrap();

    struct Server {
        out: Sender,
        user_name: String,
    }

    impl Handler for Server {
        // WebSocketとのコネクション接続を開始した場合
        fn on_open(&mut self, handshake: Handshake) -> Result<()> {
            let hashed_key: String = handshake.request.hashed_key().unwrap();
            let headers: &Vec<(String, Vec<u8>)> = handshake.request.headers();

            // ヘッダーで送信されてきたユーザ名を取得する
            for (k, v) in headers {
                if k == "User-Name" {
                    self.user_name = String::from_utf8(v.to_vec()).unwrap();
                }
            }

            // ログイン情報を接続している全てのクライアントに配信する
            println!(
                "[{}] {} Connected. hash_key: {}",
                str_datetime(),
                self.user_name,
                hashed_key
            );
            let send_message: String = format!(
                "[{}] {} Join the Chat Room.",
                str_datetime(),
                self.user_name
            );
            // self.out.broadcast(StreamMessage::new(send_message));
            return self.out.broadcast(send_message);
        }

        // メッセージを受信した場合
        fn on_message(&mut self, message: Message) -> Result<()> {
            // 受信したメッセージを接続している全てのクライアントに配信する
            let send_message: String =
                format!("[{}] {}: {}", str_datetime(), self.user_name, message);
            println!("{}", send_message);
            return self.out.broadcast(send_message);
        }

        // WebSocketとのコネクション接続が閉じた場合
        fn on_close(&mut self, code: CloseCode, reason: &str) {
            // ログイン情報を接続している全てのクライアントに配信する
            println!(
                "[{}] {} Disconnected for ({:?}) {}",
                str_datetime(),
                self.user_name,
                code,
                reason
            );
            let send_message: String = format!(
                "[{}] {} Left the Chat Room.",
                str_datetime(),
                self.user_name
            );
            let _ = self.out.broadcast(send_message);
        }
    }

    // 日付の文字列を取得
    fn str_datetime() -> String {
        // メッセージに日付を付与
        let local_datetime: DateTime<Local> = Local::now();
        let formatted_local_datetime: String = local_datetime.format("%Y-%m-%d %T").to_string();
        return formatted_local_datetime;
    }
}

