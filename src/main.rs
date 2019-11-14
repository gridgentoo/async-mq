// SPDX-License-Identifier: GPL-2.0
use futures;
use tokio;

fn main() {
    tokio::run(futures::future::lazy(|| {
        let addr: std::net::SocketAddr = "127.0.0.1:6142".parse().unwrap();
        let count = 1;
        tokio::spawn(rustmq::echo::server(&addr));
        tokio::spawn(rustmq::basic::display(count));
        tokio::spawn(rustmq::basic::better_display(count));
        tokio::spawn(rustmq::peer::hello(&addr));
        tokio::spawn(rustmq::peer::peer(&addr));
        tokio::spawn(rustmq::combinator::hello());
        tokio::spawn(rustmq::echo::client(&addr));
        tokio::spawn(rustmq::echo::client_and_then(&addr));
        tokio::spawn(rustmq::echo::client_and_then_and_then(&addr));
        Ok(())
    }))
}
