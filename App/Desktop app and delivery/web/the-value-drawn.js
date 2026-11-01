// **One way to draw a value on the page, so it reads in the order its bytes
// run** — `Desktop app and delivery` X9, its paragraph *Drawn in the order its
// bytes run*, added 2026-09-28 on the build runner's ruling that a path shown
// in another order is not the real path. Built at step 10c, dependency wave
// piece 10a.
//
// ## What went wrong without it
//
// A path such as `/root/uploads/القوام/فراء.tif`, drawn as one run of text on a
// left-to-right screen, showed as `/root/uploads/فراء/القوام.tif`: the two
// Arabic names and the `/` between them were laid out as one right-to-left
// run, so the file read as `القوام.tif` in a folder `فراء`. Seen in a throwaway
// copy on this Mac's engine, on Settings, before this file existed.
//
// ## What this file does
//
// - **The whole value is one left-to-right run, kept apart from the words
//   around it.** A `<bdi>` with `dir` set to `ltr`.
// - **A path is laid out one segment after another.** The value is cut at each
//   `/` and each `\`, and every piece between two of them is its own `<bdi>`,
//   also `ltr`, so right-to-left letters inside it read in their own direction
//   and never carry a separator or the next segment with them. The separators
//   are plain text between the pieces.
// - **A name drawn alone is a path of one segment**, so it is kept apart from
//   the text around it the same way and needs nothing else. That is why there
//   is one function here and not two.
// - **`\` counts as a separator too**, because a `.blend` records the paths of
//   the computer it was saved on, and that may be Windows. A Linux or Mac name
//   that holds a `\` has its two halves kept apart from each other; that
//   changes where they sit on the line and nothing else.
// - **Each piece is `ltr` rather than `auto`.** A name that begins in Arabic and
//   ends in `.tif` then reads with `.tif` at its right end, the way the same
//   name reads in a Files row, and the right-to-left letters still read right
//   to left. Recorded as a ruling in `Audits/Build/Desktop app and
//   delivery/Decisions record.md`, 2026-09-29.
//
// ## What it never does
//
// **It never puts anything into the value.** No mark, no space, no invisible
// character. The pieces are elements, and the text inside them, read in order,
// is the value exactly as it was handed over — so what the user copies off the
// screen is what Smoozy stored, byte for byte (X7's second and third bullets,
// and X9's *never by putting anything into it*). An isolating character put
// into the string instead would be copied and compared with it, which is why
// this cannot be done on the application side.
//
// ## The rules this rests on, quoted from their own publications
//
// - **HTML Living Standard**, section 15.3.5 *Bidirectional text*,
//   `https://html.spec.whatwg.org/multipage/rendering.html`, read 2026-09-29
//   (the page says *Last Updated 29 September 2026*). Its user-agent style
//   sheet gives both elements this file makes their isolation: *"bdi, output,
//   [dir=ltr i], [dir=rtl i], [dir=auto i] { unicode-bidi: isolate; }"*, and
//   *"[dir]:dir(ltr), bdi:dir(ltr) … { direction: ltr; }"*.
// - **HTML Living Standard**, section 4.5.24 *The bdi element*,
//   `https://html.spec.whatwg.org/multipage/text-level-semantics.html`, read
//   2026-09-29: *"The bdi element represents a span of text that is to be
//   isolated from its surroundings for the purposes of bidirectional text
//   formatting."* — and its `dir` *"defaults to auto on this element"*, which
//   is why every one made here is given `ltr` by name.
// - **CSS Writing Modes Level 3**, W3C Recommendation of 10 December 2019,
//   `https://www.w3.org/TR/css-writing-modes-3/`, read 2026-09-29, on
//   `unicode-bidi: isolate`: *"the sequence is treated as if it were a single
//   Object Replacement Character (U+FFFC)"* and *"neither is the content inside
//   the box bidi-affected by the content surrounding the box, nor is the
//   content surrounding the box bidi-affected by the content or specified
//   directionality of the box."* It also says *"This value has no effect on
//   boxes that are not inline"*, which is why the pieces are `<bdi>` — an
//   inline element — and never a block.
// - **Unicode Standard Annex #9, *Unicode Bidirectional Algorithm***, Unicode
//   18.0.0, revision 52, dated 2026-09-01, `https://www.unicode.org/reports/tr9/`,
//   read 2026-09-29. The first rule of its section 3.3.5, *Resolving Neutral
//   and Isolate Formatting Types*, is the fault: *"A sequence of NIs takes the
//   direction of the surrounding strong text if the text on both sides has the
//   same direction."* A `/` between two Arabic names is such a sequence, so it
//   went right to left with them. Section 2.4 is the repair: *"an isolate has
//   the effect of a neutral like U+FFFC OBJECT REPLACEMENT CHARACTER"* and
//   *"the text inside the isolate has no effect on the ordering of the text
//   outside it, and vice versa."* A `/` between two isolates sits between two
//   neutrals in a left-to-right run, so it stays where its bytes put it.
//
// **How the three engines lay this out is looked at on the Mac's alone and
// claimed nowhere** (X6's shape). The rules above say what an engine is
// expected to do; only the Mac's has been watched doing it.
//
// H32: the value is content, never markup — `textContent` and
// `createElement`. No word is written here (A6, X1).

/// **The characters a path is cut at.** The capture keeps each one, so the
/// pieces and the separators, read in order, are the value.
const THE_SEPARATORS = /([/\\])/;

/// **A value, drawn so it reads in the order its bytes run.** Returns the
/// element; the caller puts it where the value goes.
export function theValueDrawn(value) {
  const whole = document.createElement("bdi");
  whole.dir = "ltr";
  const text = value == null ? "" : String(value);
  const pieces = text.split(THE_SEPARATORS);
  if (pieces.length === 1) {
    whole.textContent = text;
    return whole;
  }
  for (const piece of pieces) {
    if (piece === "") continue;
    if (piece === "/" || piece === "\\") {
      whole.appendChild(document.createTextNode(piece));
      continue;
    }
    const segment = document.createElement("bdi");
    segment.dir = "ltr";
    segment.textContent = piece;
    whole.appendChild(segment);
  }
  return whole;
}

/// **The same, drawn into an element that already stands**, in place of
/// whatever it held. An empty value leaves it empty, as `textContent = ""`
/// did.
export function theValueIsDrawnInto(holder, value) {
  holder.textContent = "";
  if (value == null || value === "") return holder;
  holder.appendChild(theValueDrawn(value));
  return holder;
}
