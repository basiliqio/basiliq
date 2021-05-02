use tokio::signal::unix::{signal, Signal, SignalKind};
use tracing::info;

pub async fn wait_for_term_signal() {
    let mut alarm_signal: Signal =
        signal(SignalKind::alarm()).expect("failed to set up ALARM signal handler");
    let mut term_signal: Signal =
        signal(SignalKind::terminate()).expect("failed to set up TERM signal handler");
    let mut int_signal: Signal =
        signal(SignalKind::interrupt()).expect("failed to set up INT signal handler");
    let mut usr1_signal: Signal =
        signal(SignalKind::user_defined1()).expect("failed to set up USR1 signal handler");
    let mut usr2_signal: Signal =
        signal(SignalKind::user_defined2()).expect("failed to set up USR2 signal handler");
    let mut quit_signal: Signal =
        signal(SignalKind::quit()).expect("failed to set up QUIT signal handler");

    tokio::select! {
        _ = alarm_signal.recv() => (),
        _ = term_signal.recv() => (),
        _ = int_signal.recv() => (),
        _ = usr1_signal.recv() => (),
        _ = usr2_signal.recv() => (),
        _ = quit_signal.recv() => (),
    };
    info!("Shutting down");
}
