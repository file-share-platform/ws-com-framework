//! Communication framework to be used on the server-side of the websocket connection.

//////// Tests ////////

#[cfg(not(tarpaulin_include))]
#[doc(hidden)]
#[cfg(test)]
mod server {
    use crate::message::Message;
    use crate::{Receiver, Sender};

    #[tokio::test]
    ///Test that basic functionality works. Creates a simple unbounded channel and sends some messages down it.
    async fn basic_functionality() {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

        let mut s = Sender::new(tx); //Create a new sender over the sending stream of the websocket.

        let message: Message = Message::AuthReq(56);

        //Same syntax, except message is now of our custom type, in this way we can limit what can be
        //sent down the websockets - which should help to reduce errors.
        s.send(message).await.unwrap();

        //Close the websocket
        s.close().await.unwrap();

        let mut r = Receiver::new(rx); //Create a new reciever, which wraps over the sink of the websocket.
        while let Some(v) = r.next().await {
            //Very similar syntax to current solution
            //except that v is a custom type which we can then
            //easily match over
            assert_eq!(Message::AuthReq(56), v.unwrap());
        }
    }
}
