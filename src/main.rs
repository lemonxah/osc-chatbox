mod cxxqt_object;
mod modules;
mod orchestrator;
mod osc;
pub mod smalltext;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQmlEngine, QString, QUrl};
use std::pin::Pin;

fn main() {
    let mut app = QGuiApplication::new();

    if let Some(ref mut app_pin) = app.as_mut() {
        app_pin
            .as_mut()
            .set_organization_name(&QString::from("osc-chatbox"));
        app_pin
            .as_mut()
            .set_organization_domain(&QString::from("osc-chatbox"));
    }

    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/osc/chatbox/qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        let engine: Pin<&mut QQmlEngine> = engine.upcast_pin();
        engine
            .on_quit(|_| {
                println!("Shutting down.");
            })
            .release();
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
