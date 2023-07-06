This project extracts some information from nvidia graphics hardware and exports it to an influxdb instance.

If you want to use the project, you can configure a couple things about it:

- INFLUXDB_USERNAME
- INFLUXDB_PASSWORD
- INFLUXDB_HOSTNAME
- INFLUXDB_DATABASE

If you want to have systemd supervise it as a service, make a copy of the example unit and put it in `$HOME/.config/systemd/user`.  Configure the environment as needed then enable and start the service:

```
systemctl --user enable nv-mon.service
systemctl --user start nv-mon.service
```
