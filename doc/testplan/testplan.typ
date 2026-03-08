#show link: it => underline(text(fill: blue, it))
#set page(
  paper: "us-letter",
  margin: (x: 1in, y: 1in),
  header: [
    #set text(size: 9pt)
    #grid(
      columns: (1fr, 1fr),
      rows: (auto, auto, auto),
      gutter: 3pt,
      [*Test Plan / Report*], [],
      [Product / Team: Memori], [3/19/26],
      [Revision 1.0.0], [Revision Date: 3/19/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Test Plan / Report]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]

= System Test Scenarios

== US 1.1
As an application user, I want to be able to connect to my Memori device
using Bluetooth via simulator / desktop.

== US 1.3
As a device owner, I want to be able to display simple widgets, such as
time or weather on my Memori device.

== US 1.4
As a user, I want to have a dark mode for the display.

== US 2.1
As an application user, I want to be able to connect to my Memori device
using Bluetooth from a mobile device.

== US 2.2
As a developer, I would want to see my GitHub statistics on the Memori
device.

== US 2.2
As a student, I would want to see my canvas assignments on the Memori
device.

== US 2.3
As an efficiency seeking user, I want to be able to display multiple
widgets on my device at once.

== US 3.2
As a user, I would like sound notifications for when certain widget
information changes.

== US 3.3
As a device owner, I want to be able to have a battery enabled device
that I can charge with USB-C.

== 4.2
As a user, I need the device to stay powered for at least a week.



= Unit Tests
We don't have unit tests because We do not know how to test embedded
software "unit-wise" as rust the language does not have a test harness
for the ESP32C3. It was only after the fact that We learned about the
#link("https://crates.io/crates/embedded-hal-mock")[embedded-hal-mock],
which allows for unit testing embedded firmware.
