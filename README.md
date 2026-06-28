<div style="text-align: center;" align="center">

# `rgblamp`

### A command-line application and library to control HID LampArray devices on Linux.

[![GNU GPL v3](https://img.shields.io/badge/License-GNU_GPL_v3-blue)](./LICENSE)

</div>

# Setup

> [!NOTE]  
> If the following steps do not work, please open an issue.

## Permissions

The application needs permission to access the devices. You can check whether the permissions work by running the list command:

```sh
rgblamp list
```

If this outputs a list of devices, it means the permissions are working and you can skip to the next section.
Otherwise, follow these steps:

- Create a new file at `/etc/udev/rules.d` named `99-rgblamp.rules` and paste the following contents:

  ```
  KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0660", GROUP="rgblamp"
  ```

- Now create a user group named `rgblamp` and add yourselves to it. You need to log out and log back in for the changes to take effect.

  ```sh
  sudo groupadd rgblamp
  sudo usermod -aG rgblamp $USER
  ```

- Reload the udev rules and verify that the permissions work now.
  ```sh
  sudo udevadm control --reload-rules
  sudo udevadm trigger
  rgblamp list
  ```

## Run in background on startup

You can achieve persistent effects running in the background using systemd rules.

- Start by creating a file at `~/.config/systemd/user/rgblamp.service` and paste in the following. You can modify the `ExecStart` command to suit your preferences.

  ```sh
  [Unit]
  Description=rgblamp service
  After=network.target

  [Service]
  Type=simple
  ExecStart=/home/<YOUR_USERNAME_HERE>/.cargo/bin/rgblamp effect rainbow --retry
  Restart=on-failure
  RestartSec=5

  [Install]
  WantedBy=default.target
  ```

- Update the unit files, start the service and enable it for running at startup:
  ```sh
  systemctl --user daemon-reload
  systemctl --user enable --now rgblamp.service
  ```

# License

[GNU GPL v3](./LICENSE)
