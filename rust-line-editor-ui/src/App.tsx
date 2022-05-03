import React, { useEffect, useState } from "react";

// Styles
import "./styles.scss";

// Components
import SyntaxHighlighter from "react-syntax-highlighter";
import AceEditor from "react-ace";

const rustPaths = {
  problems: "/rust-src/src/problems.rs",
  answers: "/state",
  solution: "/rust-src/src/solution.rs",
};
type Status = "correct" | "incorrect" | "invalid";
type Editable = { answer: string; solution: string } | null;
type Section = {
  editable: Editable;
  frozen: string;
  newlines: number;
  status: Status;
  output: null | string;
};

type State =
  | {
      loaded: false;
    }
  | {
      loaded: true;
      content: Section[];
    };

const countNewLines = (src: string, pattern = /;/g) =>
  (src.match(pattern) || []).length;

function getSubsections(
  code: string
): [string, string | undefined][] {
  const subsections = code.split(/\n *\/\/ END\n/);
  return subsections.map((subsection: string) => {
    const [frozen, editable, ...rest] = subsection.split(
      /\n *\/\/ START\n/
    );
    if (rest.length === 0) {
      return [frozen, editable];
    }
    throw new Error(`is ill-formatted.`);
  });
}

export default function App() {
  const [state, setState] = useState<State>({ loaded: false });
  useEffect(() => {
    if (!state.loaded) {
      fetch(rustPaths.problems)
        .then((r) => r.text())
        .then((_) => {
          fetch(rustPaths.answers, {
            method: "GET",
            credentials: 'same-origin',
            headers: { 'Accept': 'application/json', 'Content-Type': 'application/json' }
            })
            .then((r) => r.text())
            .then((r) => JSON.parse(r))
            .then((state) =>
              fetch(rustPaths.solution)
                .then((r) => r.text())
                .then((solutions) => {
                

                  let answers = state.answers;
                  let scores = state.scores;

                  const problemSubsections = getSubsections(answers);
                  const solutionSubsections = getSubsections(solutions);
                  const content: Section[] = problemSubsections.map(
                    ([frozen, problem], i) => {
                      const [solutionFrozen, solution] = solutionSubsections[i];
                      // if (frozen !== solutionFrozen)
                      //throw new Error(`mismatch between problems and solutions:
                        // ${frozen}
                       // vs
                       // ${solutionFrozen}`);
                      const newlines =
                        problem === undefined ? 0 : countNewLines(problem);
                      const editable: Editable =
                        problem !== undefined && solution !== undefined
                          ? {
                              answer: problem,
                              solution,
                            }
                          : null;
                      return {
                        frozen,
                        editable,
                        newlines,
                        status: scores[i] == 1 ? "correct" : "incorrect",
                        output: null,
                      };
                    }
                  );
                  setState({ loaded: true, content });
              })
          );
      });
    }
  }, [state]);

  function getColor(status: "correct" | "incorrect" | "invalid") {
    switch (status) {
      case "correct":
        return "green";
      case "incorrect":
        return "black";
      case "invalid":
        return "red";
    }
  }

  if (state.loaded) {
    const lineNumbers = state.content.reduce(
      (prev, { frozen, editable }) => {
        const pattern = /\n/g;
        const frozenLineNumber =
          prev[prev.length - 1] + 1 + countNewLines(frozen, pattern);
        return prev.concat(
          editable === null
            ? [frozenLineNumber]
            : [
                frozenLineNumber,
                frozenLineNumber + 1 + countNewLines(editable.answer, pattern),
              ]
        );
      },
      [1]
    );
    const updateAnswer = (
      newlines: number,
      status: Status,
      i: number,
      editable: Editable
    ) => (input: string) => {
      const newStatus =
        countNewLines(input) > newlines ? "invalid" : "incorrect";
      if (status !== "invalid" && newStatus === "invalid") {
        alert("Too many newlines.");
      }
      const content = state.content;
      content[i] = {
        ...content[i],
        status: newStatus,
        editable: editable === null ? null : { ...editable, answer: input },
      };
      setState({ ...state, content: content });
    };

    const reset = () => {
      fetch(rustPaths.solution)
        .then((r) => r.text())
        .then((solutions) => {
          fetch(rustPaths.problems)
            .then((r) => r.text())
            .then((problems) => {
              fetch("/reset", {
                method: "POST",
                credentials: 'same-origin',
                headers: { 'Accept': 'application/json', 'Content-Type': 'application/json' },
              })
              .then((_) => {
                const problemSubsections = getSubsections(problems);
                const solutionSubsections = getSubsections(solutions);
                const content: Section[] = problemSubsections.map(
                  ([frozen, problem], i) => {
                    const [solutionFrozen, solution] = solutionSubsections[i];
                    // if (frozen !== solutionFrozen)
                    //   throw new Error(`mismatch between problems and solutions:
                    //   ${frozen}
                    //   vs
                    //   ${solutionFrozen}`);
                    const newlines =
                      problem === undefined ? 0 : countNewLines(problem);
                    const editable: Editable =
                      problem !== undefined && solution !== undefined
                        ? {
                            answer: problem,
                            solution,
                          }
                        : null;
                    return {
                      frozen,
                      editable,
                      newlines,
                      status: "incorrect",
                      output: null,
                    };
                  }
                );
                setState({ loaded: true, content });
              });
        });
    })
  };

  const checkAnswer = (status: Status, i: number) => {
      if (status === "invalid")
        alert("Remove extra newlines before compiling.");
      else {
        const src = state.content
          .map(({ frozen, editable }, j) =>
            editable === null
              ? frozen
              : `${frozen}\n${i == j ? editable.answer : editable.solution}`
          )
          .join("\n");
        
        const answers = state.content
        .map(({ frozen, editable }, j) =>
        editable === null
          ? frozen
          : `${frozen}\n// START\n${editable.answer}\n// END`
        )
        .join("\n");

        const developmentMode =
          !process.env.NODE_ENV || process.env.NODE_ENV === "development";
        const url = "/check";
        fetch(url, {
          method: "POST",
          credentials: 'same-origin',
          headers: { 'Accept': 'application/json', 'Content-Type': 'application/json' },
          body: JSON.stringify({
            to_grade: src,
            problem: i,
            edit_state: answers
          }),
        })
          .then((res) => res.text())
          .then((res) => {
            const json = JSON.parse(res);
            let newContent = state.content;

            newContent[i] = {
              ...state.content[i],
              output: json.output,
              status: json.correct ? "correct" : "incorrect",
            };
            setState({ ...state, content: newContent });
          });
      }
    };
    const sum = (array: number[]) =>
      array.reduce((prev, current) => prev + current, 0);
    const isCorrect = state.content.map(({ status }) =>
      status == "correct" ? 1 : 0
    );
    const isProblem = state.content.map(({ editable }) =>
      editable == null ? 0 : 1
    );
    return (
      <div className="App">
        <h1 className={"centerText"}>
          Grade: {sum(isCorrect)}/{sum(isProblem)}
        </h1>
        <button onClick={() => reset()}>Reset</button>
        {state.content.map(({ frozen, editable, newlines, status }, i) => {
          const output = state.content[i].output;
          const statusText = status.charAt(0).toUpperCase() + status.slice(1);
          const options = {
            enableBasicAutocompletion: false,
            firstLineNumber: lineNumbers[i * 2 + 1],
          };
          const numLines = lineNumbers[i * 2 + 2] - lineNumbers[i * 2 + 1];
          return (
            <div key={i} className={"row"}>
              <div className={"width50"}>
                <SyntaxHighlighter
                  customStyle={{ fontSize: 12 }}
                  showLineNumbers={true}
                  startingLineNumber={lineNumbers[i * 2]}
                  language="rust"
                >
                  {frozen}
                </SyntaxHighlighter>
                {editable ? (
                  <AceEditor
                    style={{ color: getColor(status) }}
                    onChange={updateAnswer(newlines, status, i, editable)}
                    value={editable.answer}
                    showGutter={true}
                    highlightActiveLine={false}
                    mode="rust"
                    height={`${16 * numLines}px`}
                    setOptions={options}
                  />
                ) : null}
              </div>
              {editable ? (
                <div className={"right column padTop"}>
                  <button onClick={() => checkAnswer(status, i)}>
                    Compile
                  </button>
                  <p>{statusText}</p>
                  {status === "correct" || output === null ? null : (
                    <div>
                      <SyntaxHighlighter
                        language={"bash"}
                        customStyle={{ fontSize: 12 }}
                      >
                        {output}
                      </SyntaxHighlighter>
                    </div>
                  )}
                </div>
              ) : null}
            </div>
          );
        })}
      </div>
    );
  }
  return <div>Loading...</div>;
}
