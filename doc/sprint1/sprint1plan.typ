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
      [*Sprint 1 Plan*], [],
      [Product / Team: Memori], [1/27/26],
      [Revision 0.1.0], [Revision Date: 1/27/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Sprint 1 Plan]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]

= Sprint Overview

== Sprint Goal

Our goals for this sprint are to set up a development environment on everyones machines,
as well as making a simulator for the device as not everyone has a board and display.
We also plan to make a Proof-of-Concept of being able to connect to the device over
bluetooth and update what its displaying on its screen through a simple app.


== Duration
*Start Date:* 1/14/26 \
*End Date:* 1/27/26\
*Sprint Length:* 2 weeks

= Sprint Tasks

== High Priority Items
+ *User Story 1.1* - As a application user, I want to be able to connect to my Memori device using bluetooth via simulator / desktop.
  - Estimated Points: 21
  - Tasks
    - Set up Bluetooth on device (2 Hours)
    - Set up a host application with BL support from desktop (3 Hours)
    - Write code to connect host application to device over bluetooth (2 Hours)
  - Total Time: 7 Hours
  - Assigned To: Preston

+ *User Story 1.2* - As a user, I want a clear and intuitive interface to interact with the Memori device through my phone.
  - Estimated Points: 8
  - Tasks
    - Set up basic Tauri Application (1 hour)
    - Add basic ui elements (1 hour)
    - Write transport layer to facilitate communication between app and device (12 Hours)
    - Hookup a transport layer to the backend (1 hour)
  - Total Time: 15 Hours
  - Assigned To: Kenric, Surendra, Preston

#pagebreak()
== Medium Priority Items
+ *User Story 1.3* - As a device owner, I want to be able to display simple widgets, such as time or weather on my Memori device.
  - Estimated Points: 8
  - Tasks
    - Familiarize with Ratatui library (2 hours)
    - Write a Weather widget (3 hours)
    - Write a Time widget (1 hour)
  - Total Time: 6 Hours
  - Assigned To: Cainan, Julian

== Low Priority Items
+ *User Story 2.4* - As a user, I want to have a dark mode for the display.
  - Estimated Points: 5
  - Tasks
    - Make the display dark. (0.5 Hours)
  - Total Time: 0.5 Hours
  - Assigned To: Surendra
= Team

#table(
  columns: (1fr, 1fr),
  [*Team Member*], [*Roles*],
  [Surendra Jammishetti], [Developer, Project Lead],
  [Kenric Tee], [App Developer],
  [Preston Clayton], [Hardware Developer],
  [Cainan Enneking], [Scrum Master, Device UI Developer],
  [Julian Montano], [Device UI Developer],
)



= Scrum Board

We are using github projects for our scrum board, which can be viewed #link("https://github.com/orgs/cse115a-Memori/projects/1")[here]

= Scrum Times
*Scrum Meetings:* Sunday 10-10:30am, Monday 1:15-2pm, Wednesday 1-2pm, Friday 2-2:30pm\
*TA meetings:* Monday 1:15 - 2pm



