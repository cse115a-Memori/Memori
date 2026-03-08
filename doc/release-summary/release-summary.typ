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

// List the major bugs (you can reference your Test Report), omissions (missing
// functionality, edge cases that are not handled), design shortcuts (e.g. hard
// coded data), etc.
= Known Issues


// Provide a list of the high priority user stories and bug fixes that can
// serve as a guide for a follow-on project
= Product Backlog



