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
// These are the activities or actions the team determined they
// should stop doing. This is the answer to the question, "What things should we stop
// doing?" The items should take the form of a brief description of what the team wants to
// stop doing, followed by a brief explanation. If there are no items, this section should
// describe why the team is completely satisfied with their current process.
// Examples: The team should stop holding daily scrum meetings at 7am in the morning,
// because nobody can make that meeting time. The team should stop allowing daily scrum

// meetings to go over 15 minutes, because the meetings are less effective that wa



- Stop fumbling around on meeting times, and discuss future meeting times when everyone is together.
  We want to stop doing this because it leads to big windows of wasted time when people aren't all together.


- Having unstructured meetings. Not everyone has a clear understanding of meeting structure.
  Our meetings consisted of jumbled explanations of what people were doing and what they plan on doing,
  but it usually consisted of a lot of confusing jumping around that would lead to inaction or misunderstanding.

- Dont push code to main without someone reviewing. This is a scary process that can lead to bugs and errors that
  are difficult to fix later on.

= Actions to Start Doing
// These are the activities or actions the team would like to start
// doing to improve their development process. This is the answer to the question, "What
// should we start doing?" The items should take the form of a brief description of what the
// team wants to start doing, followed by a brief explanation.
// Examples: The team should schedule more group work sessions, since these are very
// effective at getting work done. The team should be more accurate at estimating work
// tasks, since tasks were consistently under-estimated last sprint.


- Start actually implementing standups at the start of the meeting where everyone is expected to show
  what they've been doing. Note: This should only happen, or re-happen, once everyone is there, that way
  everyone can see what everyone else has been up to. Adding structure to our meetings will lead to the team
  being more cohesive and on the same page, leading to more productivity.

- Push code frequently to the repository so everyone can be up to date with other's work. Often times, members would
  like to go back and see the code that others have written. This will also help prevent merge conflicts.

- Try writing more tests. This is something that we know we should do to prevent headache in the future.

- Add story points to kanban board / github issues project so we can create a burnup chart. This would provide us with better insight
  into how much work we can accomplish in a given sprint.

= Actions to Keep Doing
//This is the answer to the question, "What is working well that we
//should continue to do?" The items should take the form of a brief description of what the
//team wants to start doing, followed by a brief explanation.
- Planning poker for estimating task story points. This has been a great way to ensure that everyone is on the same page and that
  we are not overestimating or underestimating the effort required to complete a task.

- Communicating efficiently through online channels and taking initiative to meet up when needed to accomplish tasks efficiently.
  This has been a great way to communicate quickly when needed.

- Zed collaboration feature for peer programming / writing documents. We use this religiously to code with each other, and it makes
  it easy to share code and collaborate on documents.

- Keep using the kanban github project tracker. The tracker clearly lays out what needs to be done, and what we have done.


= Work Completion
// This is a list of the user stories that were completed
// during the previous sprint, and a list of the user stories not completed during this sprint
// (but which were part of this sprint, and were in the sprint plan).
== Completed
*User Story 1.1* - As a application user, I want to be able to connect to my Memori device using bluetooth via simulator / desktop.
  - Assigned To: Preston, Surendra
*User Story 1.3* - As a device owner, I want to be able to display simple widgets, such as time or weather on my Memori device.
- Assigned To: Cainan, Julian
*User Story 2.4* - As a user, I want to have a dark mode for the display.
  - Assigned To: Surendra

== Incomplete
*User Story 1.2* - As a user, I want a clear and intuitive interface to interact with the Memori device through my phone.
- Assigned To: Kenric, Surendra, Preston
== Completion rate
// This section should report the following: total number of user
// stories completed during the prior sprint. Total number of estimated ideal work hours
// completed during the prior sprint. Total number of days during the prior sprint. For the
// previous sprint, the user stories/day and ideal work hours/day figures should be reported.
// For sprints past the first sprint, this section should also provide the average user
// stories/day and average ideal work hours/day figures computed across all sprints to date.
// The final sprint burnup chart for the previous sprint should be available for viewing in the
// lab and an email of this chart sent to the TA/prof.


Completion rate: 75 %

Completed 3 user stories.

Estimated Work hours Completed : 13.5

Total number of days during the prior sprint : 13

User stories / day : 0.23

Work hours / day : 1.0384615384615385



= Burnup Chart
#image("Story Points Burn.png")
// Get a burnup chart from github
