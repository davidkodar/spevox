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
    run_profile_bridge_at(sender, PROFILE_BUS_NAME).await
}

async fn run_profile_bridge_at(
    sender: mpsc::Sender<ActiveApplication>,
    bus_name: &str,
) -> zbus::Result<()> {
    let _connection = zbus::connection::Builder::session()?
        .name(bus_name)?
        .serve_at("/Profiles", ProfileBridge { sender })?
        .build()
        .await?;
    std::future::pending::<()>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use super::{ActiveApplication, run_profile_bridge_at};

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
        let bus_name = format!(
            "io.github.davidkodar.FluidVoiceLinux.Test{}",
            std::process::id()
        );
        let bridge_name = bus_name.clone();
        let bridge = tokio::spawn(async move { run_profile_bridge_at(sender, &bridge_name).await });
        let connection = zbus::Connection::session().await.expect("session bus");
        let dbus = zbus::fdo::DBusProxy::new(&connection)
            .await
            .expect("D-Bus proxy");
        for _ in 0..50 {
            if dbus
                .name_has_owner(bus_name.as_str().try_into().expect("bus name"))
                .await
                .expect("name owner query")
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert!(
            dbus.name_has_owner(bus_name.as_str().try_into().expect("bus name"))
                .await
                .expect("final name owner query"),
            "profile test bridge did not acquire its isolated bus name"
        );
        let proxy = zbus::Proxy::new(
            &connection,
            bus_name.as_str(),
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
        let reported = tokio::time::timeout(std::time::Duration::from_secs(2), receiver.recv())
            .await
            .expect("timed out waiting for bridge event")
            .expect("bridge event");
        assert_eq!(reported.resource_class, "org.kde.konsole");
        assert_eq!(reported.title, "Integration Test");
        bridge.abort();
    }
}
