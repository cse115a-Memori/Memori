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
      [*Sprint 2 Report*], [],
      [Product / Team: Memori], [2/24/26],
      [Revision 0.1.0], [Revision Date: 2/25/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Sprint 3 Report]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]



= Actions to Stop Doing
// These are the activities or actions the team determined they
// should stop doing. This is the answer to the question, "What things should we stop
// doing?" The items should take the form of a brief description of what the team wants to
// stop doing, followed by a brief explanation. If there are no items, this section should
// describe why the team is completely satisfied with their current process.
// Examples: The team should stop holding daily scrum meetings at 7am in the morning,
// because nobody can make that meeting time. The team should stop allowing daily scrum
// meetings to go over 15 minutes, because the meetings are less effective that wa
None as this is the last sprint, and we are satisfied with our current process.

= Actions to Start Doing
// These are the activities or actions the team would like to start
// doing to improve their development process. This is the answer to the question, "What
// should we start doing?" The items should take the form of a brief description of what the
// team wants to start doing, followed by a brief explanation.
// Examples: The team should schedule more group work sessions, since these are very
// effective at getting work done. The team should be more accurate at estimating work
// tasks, since tasks were consistently under-estimated last sprint.

Split up larger user stories into smaller, more manageable tasks.


= Actions to Keep Doing
//This is the answer to the question, "What is working well that we
//should continue to do?" The items should take the form of a brief description of what the
//team wants to start doing, followed by a brief explanation.
- Keep putting up PR's as this helps us become more aware of the work being done
  whenever we need the understanding.
- Keep using discord channels to communicate with each other whenever we run into issues working on user stories.


= Work Completion
// This is a list of the user stories that were completed
// during the previous sprint, and a list of the user stories not completed during this sprint
// (but which were part of this sprint, and were in the sprint plan).
== Completed
+ *User Story 3.1* As a user, I want to be able to choose the widgets that are displayed on my device.
- Estimated Points: 8
  - Tasks
    - Implement send state button which sends the selected widgets to the device (2 Hours) (Kenric)
    - Total Time: 2 Hours

+ *User Story 2.4* {OVERFLOW} - As an efficiency seeking user, I want to be able to display multiple widgets on my device at once.
  - Estimated Points: 13
  - Tasks
    - Implement visuals for different widgets on the app (5 Hours) (Kenric)
      - per widget configuration
  - Total Time: 5 Hours

+ *User Story 2.3* {OVERFLOW} - As a student, I would want to see SCMETRO bus information on my device
  - Estimated Points: 1
    - Tasks
      - merge finished code (1 hour) (Julian)
    - Total Time: 1 Hours

+ *User Story 2.2* {OVERFLOW} - As a developer, I would want to see my github statistics on the Memori device.
    - Estimated Points: 5
    - Tasks
      - github widget data logic (3 hour) (Cainan)
    - Total Time: 3 Hours

== Incomplete
+ *User Story 3.2* - As a device owner, I want my device to be portable.
  - Estimated Points: 34
  - Tasks
    - Print and assemble PCB (8 hours) (Surendra, Preston, Julian, Kenric, Cainan)
  - Total Time: 8 Hours

  + *User Story 3.3* - As a user, I want my widgets to display accurate updated information.
    - Estimated Points: 8
    - Tasks
      - Implement update function for each widget and put that in the app (2 hours) (Preston)
    - Total Time: 2 Hours

== Completion rate
// This section should report the following: total number of user
// stories completed during the prior sprint. Total number of estimated ideal work hours
// completed during the prior sprint. Total number of days during the prior sprint. For the
// previous sprint, the user stories/day and ideal work hours/day figures should be reported.
// For sprints past the first sprint, this section should also provide the average user
// stories/day and average ideal work hours/day figures computed across all sprints to date.
// The final sprint burnup chart for the previous sprint should be available for viewing in the
// lab and an email of this chart sent to the TA/prof.


Completion rate: 66%

Completed 4 user stories!

Estimated Work hours Completed : 11

Total number of days during the prior sprint : 14

User stories / day : 0.29

Work hours / day : 0.79

= Burnup Chart
#image("sprint4burnup.png")
