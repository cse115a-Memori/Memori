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

== US 2.3
As a UCSC student, I would want to see bus information on the Memori device.

== US 2.4
As an efficiency seeking user, I want to be able to display multiple
widgets on my device at once

== US 3.1
As a user, I want to be able to choose the widgets that are displayed on my device.

== US 3.3
As a user, I want my widgets to display accurate updated information.

== US 4.1
As a device owner, I want to be able to have a battery enabled device
that I can charge with USB-C.

== US 4.2
As a user, I need the device to stay powered for at least a week.

// List the major bugs (you can reference your Test Report), omissions (missing
// functionality, edge cases that are not handled), design shortcuts (e.g. hard
// coded data), etc.
= Known Issues
- connection state desync issues on app 
  - if device disconnects from app, for example by exiting the app and resuming it, that's not reflected in the state
- if the user accidentally clicks onboarding after having already onboarded, they have to type
  in a pairing code and attempt to connect regardless of if they have already paired.
- Bus widget doesn't work
- Twitch widget doesn't work
- After the user has already moved a widget tile to the frame,
  the "Moving: {}" text remains visible.
- The clock widget will sometimes display the wrong time.
- The smallest layout for the weather widget does not properly render.
  it overflows into a different widget's box.
- After a user selects a different preference for github widget repo and updates to device,
  the device takes at least 10 seconds to reflect the new repo.
- After a user selects a different preference for github widget repo and updates to device,
  the device will then revert back to the previous repo data upon a refresh request.


// Provide a list of the high priority user stories and bug fixes that can
// serve as a guide for a follow-on project
= Product Backlog
- Iron out state synchronization issues over BLE
- Give user more insight into the state of the system
- Fix twitch widget data fetching
- Revise bus widget to send less data to device
- Ensure a proper user interface flow that isn't so much a dev test environment



