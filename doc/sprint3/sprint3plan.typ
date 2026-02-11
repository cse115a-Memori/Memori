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
      [*Sprint 3 Plan*], [],
      [Product / Team: Memori], [2/11/26],
      [Revision 0.1.0], [Revision Date: 2/11/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Sprint 3 Plan]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]

= Sprint Overview

== Sprint Goal

The main goal of this sprint is to tie up all the loose ends from last sprint and reevaluate how practical it is to
create a finished hardware product.

== Duration
*Start Date:* 2/12/26 \
*End Date:* 2/26/26\
*Sprint Length:* 2 weeks

= Sprint Tasks



== High Priority Items
+ *User Story 2.1* {OVERFLOW} - As an application user, I want to be able to connect to my Memori device using bluetooth from a mobile device.
  - Estimated Points: 3
  - Tasks
    - Verify that btleplug works on ios (1 Hour) (Preston)
  - Total Time: 3 Hours

+ *User Story 3.1* As a user, I want to be able to choose the widgets that are displayed on my device.
- Estimated Points: 8
  - Tasks
    - Create widget selection screen on the app's frontend (5 Hours) (Kenric)
    - Implement send state button which sends the selected widgets to the device (2 Hours) (Kenric)
    - Total Time: 3 Hours

+ *User Story 2.4* {OVERFLOW} - As an efficiency seeking user, I want to be able to display multiple widgets on my device at once.
  - Estimated Points: 13
  - Tasks
    - Implement visuals for different widgets on the app (5 Hours) (Kenric)
      - per widget configuration
    - Implement ui to configure different widgets in different layours (5 hours) (Kenric)
      - layout configuration
  - Total Time: 10 Hours

== Medium Priority Items
+ *User Story 3.3* - As a user, I want my widgets to display accurate updated information.
  - Estimated Points: 8
  - Tasks
    - Move host request handler to be channel based. (expose state to host-ble command handling) (2 hours) (Preston)
    - Implement update function for each widget and put that in the app (2 hours) (Preston)
  - Total Time: 4 Hours
+ *User Story 2.2* {OVERFLOW} - As a developer, I would want to see my github statistics on the Memori device.
  - Estimated Points: 5
  - Tasks
    - github widget data logic (3 hour) (Cainan)
  - Total Time: 3 Hours

+ *User Story 2.3* {OVERFLOW} - As a student, I would want to see SCMETRO bus information on my device
- Estimated Points: 1
  - Tasks
    - merge finished code (1 hour) (Julian)
  - Total Time: 1 Hours

== Low Priority Items
+ *User Story 3.2* - As a device owner, I want my device to be portable.
  - Estimated Points: 34
  - Tasks
    - Decide on what battery to get (2 hours) (Surendra, Preston)
    - Design a PCB to utilize the battery (10 hours) (Surendra)
    - Print and assemble PCB (8 hours) (Cainan)
  - Total Time: 20 Hours


= Team

#table(
  columns: (1fr, 1fr),
  [*Team Member*], [*Roles*],
  [Surendra Jammishetti], [Developer, Project Lead],
  [Kenric Tee], [App Developer],
  [Preston Clayton], [Scrum Master, Hardware Developer],
  [Cainan Enneking], [Device UI Developer],
  [Julian Montano], [Device UI Developer],
)


= Scrum Board

We are using github projects for our scrum board, which can be viewed #link("https://github.com/orgs/cse115a-Memori/projects/1")[here]

= Scrum Times
*Scrum Meetings:* Sunday 10-10:30am, Monday 1:15-2pm, Wednesday 1-2pm, Friday 2-2:30pm\
*TA meetings:* Monday 1:15 - 2pm
