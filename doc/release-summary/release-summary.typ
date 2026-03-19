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
      [*Release Summary*], [],
      [Product / Team: Memori], [3/19/26],
      [Revision 1.0.0], [Revision Date: 3/19/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Release Summary]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]

//NOTE: There will be no penalty for system failures during the project
// review that are due to known problems.


// Give a list of key user stories with their acceptance criteria
// that can serve as a guide for an acceptance test.
= Key User Stories

== US 1.2
As an application user, I want to be able to connect to my Memori device
using Bluetooth via simulator / desktop.

*Acceptance Criteria:*
- *Given* the desktop app is open and a Memori device is powered on and in range, *when* the user inputs the code displayed on it, *then* the device connects within 10 seconds.
- *Given* a connection is established, *when* the connection drops *then* the app reflects the disconnected state accurately and prompts the user to reconnect.

== US 1.3
As a device owner, I want to be able to display simple widgets, such as
time or weather on my Memori device.

*Acceptance Criteria:*
- *Given* the device is connected and powered on, *when* the user adds a clock widget, *then* it renders on the device display showing the correct local time, updating each minute.
- *Given* the device is connected and powered on, *when* the user adds a weather widget, *then* it renders current conditions and temperature without overflowing its allocated area.

== US 1.4
As a user, I want to have a dark mode for the display.

*Acceptance Criteria:*
- *Given* The memori device is on, *when* at all times, *then* the device UI displays white text on a black background.

== US 2.1
As an application user, I want to be able to connect to my Memori device
using Bluetooth from a mobile device.

*Acceptance Criteria:*
- *Given* the mobile app is open with Bluetooth enabled and a Memori device is nearby, *when* the user inputs the code displayed on the device, *then* the Memori device can be connected to succesfully.
- *Given* the app is already paired and returns from the background with a dropped connection, *when* the user opens the app, *then* the app correctly shows a disconnected state and offers to reconnect.

== US 2.2
As a developer, I would want to see my GitHub statistics on the Memori
device.

*Acceptance Criteria:*
- *Given* the user has added the GitHub widget and set a repo preference, *when* the widget loads, *then* it displays accurate stats for the selected repo.
- *Given* the user changes the repo preference and pushes an update, *when* the update is received, *then* the device reflects the new repo data within a few seconds.

== US 2.3
As a UCSC student, I would want to see bus information on the Memori device.

*Acceptance Criteria:*
- *Given* the user has added the bus widget and selected a stop, *when* the widget loads, *then* it displays upcoming arrival times sourced from live UCSC transit data.
- *Given* the widget is displayed, *when* arrival times change, *then* the widget reflects updated times within its refresh interval.

== US 2.4
As an efficiency seeking user, I want to be able to display multiple
widgets on my device at once.

*Acceptance Criteria:*
- *Given* the user has added multiple widgets to the layout, *when* the layout is pushed to the device, *then* all widgets render simultaneously without overlap or clipping.
- *Given* multiple widgets are active, *when* one widget refreshes, *then* it updates independently without disrupting the other displayed widgets.

== US 3.1
As a user, I want to be able to choose the widgets that are displayed on my device.

*Acceptance Criteria:*
- *Given* the user is on the widget selection screen, *when* they browse available widgets and layout, *then* all available widgets are shown and are able to be dragged to the active frame in arrangement specified by the layout.
- *Given* the user adds or removes a widget and clicks update, *when* the layout is pushed to the device, *then* only the selected widgets appear on the device display.

== US 3.3
As a user, I want my widgets to display accurate updated information.

*Acceptance Criteria:*
- *Given* a widget is displayed on the device, *when* its underlying data changes, *then* the widget reflects the new data within its defined refresh interval.

== US 4.1
As a device owner, I want to be able to have a battery enabled device
that I can charge with USB-C.

*Acceptance Criteria:*
- *Given* the device battery is low or depleted, *when* the user connects a standard USB-C cable to a power source, *then* the device begins charging.

== US 4.2
As a user, I need the device to stay powered for at least a day.

*Acceptance Criteria:*
- *Given* the device is fully charged and in normal use (widgets active, Bluetooth connected), *when* 24 hours have elapsed, *then* the device is still powered on with remaining battery charge.

// List the major bugs (you can reference your Test Report), omissions (missing
// functionality, edge cases that are not handled), design shortcuts (e.g. hard
// coded data), etc.
= Known Issues
- Connection state de-sync issues on app.
  - Connection state is dropped when mobile app gets backgrounded. This isn't reflected properly in the app's internal state.

// - If the user accidentally clicks onboarding after having already
//   onboarded, they have to type in a pairing code and attempt to connect
//   regardless of if they have already paired.

// - Bus widget doesn't work

// - Twitch widget doesn't't work.

- After the user has already moved a widget tile to the frame,
  the "Moving: {}" text remains visible.

// - The clock widget will sometimes display the wrong time.

- The smallest layout for the weather widget does not properly render.
  it overflows into a different widget's box.

- After a user selects a different preference for GitHub widget repo and updates to device,
  the device takes at least 10 seconds to reflect the new repo.

- After a user selects a different preference for GitHub widget repo and updates to device,
  the device will then revert back to the previous repo data upon a refresh request.

- Starting onboarding after it has already been completed will not allow pairing if a device is still connected.

- When the app and device state is de-sync'd, the app needs to be restarted and the esp needs to be hard reset.

- When the weather widget is in a quadrant, it overflows it's text to the left.

- Sometimes the device drops the connection, we couldn't figure out why, needs to reset the app.


// Provide a list of the high priority user stories and bug fixes that can
// serve as a guide for a follow-on project
= Product Backlog
- Iron out state synchronization issues over BLE
- Give user more insight into the state of the system
- Fix twitch widget data fetching
- Revise bus widget to send less data to device
- Ensure a proper user interface flow that isn't so much a dev test environment



