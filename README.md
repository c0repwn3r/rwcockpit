# rwcockpit

A framework for building real-world replica cockpits to control X-Plane 12

## components

in order, from x-plane to the individual panel firmware:

- RWCockpitPlugin: an X-Plane plugin to communicate with
- cockpitd: the host software which manages communication to
- hubfw: a small RPi Pico based board which exposes an interface for cockpitd to communicate with the i2c buses, to communicate with
- panelfw: the firmware running on each individual panel, which exposes input devices over an i2c bus
