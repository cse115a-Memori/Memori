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
      [*Sprint 1 Report*], [],
      [Product / Team: Memori], [1/27/26],
      [Revision 0.1.0], [Revision Date: 1/27/26],
    )
    #line(length: 100%, stroke: 0.5pt)
  ],
)

#set text(font: "Liberation Sans", size: 11pt)
#set par(justify: true)

#align(center)[
  #text(size: 18pt, weight: "bold")[Sprint 1 Report]
  #v(0.5em)
  #text(size: 14pt)[Memori]
  #v(1em)
]



= Actions to Stop Doing
These are the activities or actions the team determined they
should stop doing. This is the answer to the question, "What things should we stop
doing?" The items should take the form of a brief description of what the team wants to
stop doing, followed by a brief explanation. If there are no items, this section should
describe why the team is completely satisfied with their current process.
Examples: The team should stop holding daily scrum meetings at 7am in the morning,
because nobody can make that meeting time. The team should stop allowing daily scrum
meetings to go over 15 minutes, because the meetings are less effective that wa

= Actions to Start Doing
These are the activities or actions the team would like to start
doing to improve their development process. This is the answer to the question, "What
should we start doing?" The items should take the form of a brief description of what the
team wants to start doing, followed by a brief explanation.
Examples: The team should schedule more group work sessions, since these are very
effective at getting work done. The team should be more accurate at estimating work
tasks, since tasks were consistently under-estimated last sprint.

= Actions to Keep Doing
This is the answer to the question, "What is working well that we
should continue to do?" The items should take the form of a brief description of what the
team wants to start doing, followed by a brief explanation.


= Work Completion
// This is a list of the user stories that were completed
// during the previous sprint, and a list of the user stories not completed during this sprint
// (but which were part of this sprint, and were in the sprint plan).


== Completed

== Incomplete

== Completion rate
// This section should report the following: total number of user
// stories completed during the prior sprint. Total number of estimated ideal work hours
// completed during the prior sprint. Total number of days during the prior sprint. For the
// previous sprint, the user stories/day and ideal work hours/day figures should be reported.
// For sprints past the first sprint, this section should also provide the average user
// stories/day and average ideal work hours/day figures computed across all sprints to date.
// The final sprint burnup chart for the previous sprint should be available for viewing in the
// lab and an email of this chart sent to the TA/prof.


= Burnup Chart
// Get a burnup chart from github


