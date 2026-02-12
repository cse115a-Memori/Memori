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
      [*Sprint 2 Plan*], [],
      [Product / Team: Memori], [1/27/26],
      [Revision 0.1.0], [Revision Date: 1/27/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Sprint 2 Plan]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]

= Sprint Overview

== Sprint Goal

The goal of this sprint is to actually connect to the Memori device through the user's phone. We would
also like to establish a way to display several widgets on the device at once such as github statistics
and canvas assignments.

== Duration
*Start Date:* 1/28/26 \
*End Date:* 2/11/26\
*Sprint Length:* 2 weeks

= Sprint Tasks


// === Sprint 2, 34 pts

// + {High} User Story 2.1 [8]: As a application user, I want to be able to connect to my Memori device using bluetooth from a mobile device.
// + {High} User Story 2.2 [8]: As a developer, I would want to see my github statistics on the Memori device.
// + {Medium} User Story 2.3 [13]: As a student, I would want to see my canvas assignments on the Memori device.
// + {Medium} User Story 2.4 [5]: As an efficiency seeking user, I want to be able to display multiple widgets on my device at once.

== High Priority Items
+ *User Story 2.1* - As an application user, I want to be able to connect to my Memori device using bluetooth from a mobile device.
  - Estimated Points: 8
  - Tasks
    - Set up bluetooth on Tauri (4 Hours) (Preston)
    - Write app-ui for pairing with device (3 Hours)(Kenric)
    - Verify connection between phone and device (1 Hour) (Preston)
  - Total Time: 7 Hours

+ *User Story 2.4* - As an efficiency seeking user, I want to be able to display multiple widgets on my device at once.
  - Estimated Points: 34
  - Tasks
    - Develop a shared layout / widget representation between device and app (7 Hours) (Surendra)
    - Implement visuals for different widgets on the app (5 Hours) (Kenric)
      - per widget configuration
    - Implement ui to configure different widgets in different layours (5 hours) (Kenric)
      - layout configuration
    - Ratatui logic to parse layout representation / widgets and put them in the right place (5 hours) (Cainan)
  - Total Time: 17 Hours
#pagebreak()
== Medium Priority Items
// + {Medium} User Story 2.3 [13]: As a student, I would want to see my canvas assignments on the Memori device.
// + {Medium} User Story 2.4 [5]: As an efficiency seeking user, I want to be able to display multiple widgets on my device at once.
+ *User Story 2.2* - As a developer, I would want to see my github statistics on the Memori device.
  - Estimated Points: 13
  - Tasks
    - Establish universal widget update framework (5 Hour) (Surendra)
    - github widget data logic (3 hour) (Cainan)
    - github widget ui (1 hour) (Cainan)
  - Total Time: 9 Hours
+ *User Story 2.3* - As a student, I would want to see bus information on the Memori device.
  - Estimated Points: 13
  - Tasks
    - santa cruz metro real time logic (5 hour) (Julian)
    - bus widget map ui (2 hour) (Julian)
  - Total Time: 7 Hours


= Team

#table(
  columns: (1fr, 1fr),
  [*Team Member*], [*Roles*],
  [Surendra Jammishetti], [Developer, Project Lead],
  [Kenric Tee], [Scrum Master,App Developer],
  [Preston Clayton], [Hardware Developer],
  [Cainan Enneking], [Device UI Developer],
  [Julian Montano], [Device UI Developer],
)


= Scrum Board

We are using github projects for our scrum board, which can be viewed #link("https://github.com/orgs/cse115a-Memori/projects/1")[here]

= Scrum Times
*Scrum Meetings:* Sunday 10-10:30am, Monday 1:15-2pm, Wednesday 1-2pm, Friday 2-2:30pm\
*TA meetings:* Monday 1:15 - 2pm
