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

Scenario 1: Onboarding
  1. Start memori app;
    + Note down the pairing code on the display.
    + Type in pairing code into the app dialogue box.
    + Hit connect.
  2. Type your name into the name field;
    + Click confirm.
    + User should see their name displayed on the device.

== US 1.3
As a device owner, I want to be able to display simple widgets, such as
time or weather on my Memori device.

Scenario 1: Displaying the Time Widget (Pass/Fail)
  1. Start memori app;
    + If first time onboarding, complete onboarding as described above.
  2. Navigate to device page;
    + Drag the time widget on to the layout pane.
  3. Click set state;
    + User should see the current time displayed on the device.


== US 1.4
As a user, I want to have a dark mode for the display.

Scenario 1: Seeing the Dark Mode (Pass/Fail)
1. Power the memori device;
  + User should see a white font pairing code (there is no light mode)


== US 2.1
As an application user, I want to be able to connect to my Memori device
using Bluetooth from a mobile device.

Scenario 1: Connect via Bluetooth For First Time (Pass/Fail)
  1. Start memori app;
    + Note pairing code on device display.
  2. Type in pairing code into the app dialogue box;
  3. Tap "Pair" button;
    + User should see their name displayed on the device.

Scenario 2: Connect via Bluetooth For Non First Time (Pass/Fail)
  1. Start memori app;
  2. Wait 1 minute for device to automatically connect to the app;
  3. If not automatically connected, tap the Connect button;
    + users should be able to navigate to the device page successfully.

== US 2.2
As a developer, I would want to see my GitHub statistics on the Memori
device.

Scenario 1: Display Github Stats (Pass/Fail)
1. start memori app;
  + if first time onboarding, complete onboarding as described above.
2. Navigate to login page;
  + If not logged in, click "Connect" button next to the github logo.
  + Enter github account login information.
  + Click the authorize button to give memori access to your account.
  + Close the browser windo and return to app.
  + Verify that github button displays "Connected".
3. Navigate to device page;
  + Tap the 3 dots in the corner of the github widget tile.
  + Select the repository for which you want to see statistics.
  + Click out of the repo selection drawer.
4. Tap the "Update Device" button.
  + User should see the statistics for the selected repository displayed on the device.

== US 2.3
As an efficiency seeking user, I want to be able to display multiple
widgets on my device at once

Scenario 1: Display Multiple Widgets (Pass/Fail)
1. start memori app;
  + if first time onboarding, complete onboarding as described above.
2. Navigate to device page;
  + tap the "layout" dropdown button.
  + select a layout other than Fullscreen.
3. Drag the required amount of widget tiles on to the layout panel;
  + Tap the "Update Device" button.
  + User should see multiple widgets displayed on the device.

== US 2.3
As a student, I would want to see bus information on the Memori device.

Scenario 1: Display Bus Routs/Time (Pass/Fail)
1. start memori app;
  + if first time onboarding, complete onboarding as described above.
  + user grant permission to access location
2. Show bus widget with current route and time;

== US 3.1
As a user, I want to be able to choose the widgets that are displayed on my device.

Scenario 1: Choosing Widget (Pass/Fail)
1. start memori app
  + if first time onboarding, complete onboarding as described above.
2. Navigate to device page;
  + drag the desired widgets into the layout
  + user tries to update widgets to device after filling out all widget slots
    + updates widgets on memori device display
  + user tries to update widgets to device without filling out all widget slots
    + fails, error message saying stating "Cannot flash..."


== US 3.3
As a user, I want my widgets to display accurate updated information.

1. Refer to Scenario 2.2 to display the github widget
2. on the account you logged in with create a new commit to the repository selected.
3. on the device verify that within around a minute the widget's commit graph shows the new commit
4. SUCCESS!

== US 4.1
As a device owner, I want to be able to have a battery enabled device
that I can charge with USB-C.

+ look at device, verify that it has a usb-c port on the side.
+ plug it in and bear witness to the little red led lighting up on the main board
+ SUCCESS

== US 4.2
As a user, I need the device to stay powered for at least a day.

+ Run the device for a day without charging.
+ Verify that the app can still connect to it after a day
+ SUCCESS



= Unit Tests
We don't have unit tests because We do not know how to test embedded
software "unit-wise" as rust the language does not have a test harness
for the ESP32C3. It was only after the fact that We learned about the
#link("https://crates.io/crates/embedded-hal-mock")[embedded-hal-mock],
which allows for unit testing embedded firmware.

However, for the hardware side of the projcet, KiCad comes with an
ERC (electrical rule checker) that ensures that PCB traces are routed
according to the schematic and dont break well known rules of
PCB routing.
