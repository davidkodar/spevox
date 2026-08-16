use tokio::sync::mpsc;

pub const PROFILE_BUS_NAME: &str = "io.github.davidkodar.FluidVoiceLinux.Profiles";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveApplication {
    pub resource_class: String,
    pub title: String,
}

struct ProfileBridge {
    sender: mpsc::Sender<ActiveApplication>,
}

#[zbus::interface(name = "io.github.davidkodar.FluidVoiceLinux.Profiles")]
impl ProfileBridge {
    async fn active_application(&self, resource_class: &str, title: &str) {
        let application = ActiveApplication {
            resource_class: resource_class.trim().chars().take(256).collect(),
            title: title.trim().chars().take(512).collect(),
        };
        if !application.resource_class.is_empty() {
            self.sender.send(application).await.ok();
        }
    }
}

/// Owns the local session-bus endpoint used by the opt-in `KWin` script.
///
/// The script reports only the active window's application class and title;
/// no content, keyboard events, or process environment crosses this boundary.
///
/// # Errors
/// Returns a D-Bus error if the session bus name or object path cannot be owned.
pub async fn run_profile_bridge(sender: mpsc::Sender<ActiveApplication>) -> zbus::Result<()> {
    let _connection = zbus::connection::Builder::session()?
        .name(PROFILE_BUS_NAME)?
        .serve_at("/Profiles", ProfileBridge { sender })?
        .build()
        .await?;
    std::future::pending::<()>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::{ActiveApplication, PROFILE_BUS_NAME, run_profile_bridge};

    #[test]
    fn active_application_is_value_comparable() {
        let application = ActiveApplication {
            resource_class: "org.kde.konsole".into(),
            title: "Terminal".into(),
        };
        assert_eq!(application.resource_class, "org.kde.konsole");
    }

    #[tokio::test]
    async fn local_bridge_receives_bounded_application_identity() {
        let (sender, mut receiver) = mpsc::channel(1);
        let bridge = tokio::spawn(run_profile_bridge(sender));
        let connection = zbus::Connection::session().await.expect("session bus");
        let dbus = zbus::fdo::DBusProxy::new(&connection)
            .await
            .expect("D-Bus proxy");
        for _ in 0..50 {
            if dbus
                .name_has_owner(PROFILE_BUS_NAME.try_into().expect("bus name"))
                .await
                .expect("name owner query")
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        let proxy = zbus::Proxy::new(
            &connection,
            PROFILE_BUS_NAME,
            "/Profiles",
            "io.github.davidkodar.FluidVoiceLinux.Profiles",
        )
        .await
        .expect("profile bridge proxy");
        proxy
            .call::<_, _, ()>(
                "ActiveApplication",
                &("org.kde.konsole", "Integration Test"),
            )
            .await
            .expect("report active application");
        let reported = receiver.recv().await.expect("bridge event");
        assert_eq!(reported.resource_class, "org.kde.konsole");
        assert_eq!(reported.title, "Integration Test");
        bridge.abort();
    }
}
