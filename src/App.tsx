import React, { useEffect, useState } from "react";
import "./App.css";
import init, { Move, possible_moves } from "./pkg";
import blackCrown from "./blackCrown.svg";
import whiteCrown from "./whiteCrown.svg";

interface IFigure {
  color: "black" | "white";
  kind: "man" | "king";
}

function Figure(props: IFigure) {
  return (
    <div className="figure" style={{ backgroundColor: props.color }}>
      {props.kind === "king" && (
        <img
          src={props.color === "black" ? whiteCrown : blackCrown}
          alt="crown"
          className="crown"
        />
      )}
    </div>
  );
}

interface ISquareProps {
  value: IFigure | null;
  onClick: React.MouseEventHandler<HTMLButtonElement>;
  color: string;
}

function Square(props: ISquareProps) {
  return (
    <button
      className="square"
      style={{ backgroundColor: props.color }}
      onClick={props.onClick}
    >
      {props.value && (
        <Figure color={props.value.color} kind={props.value.kind} />
      )}
    </button>
  );
}

interface IBoardProps {
  figureMap: Map<number, IFigure>;
  possibleMoves: Move[];
  onClick: (squareNo: number, squareValue: IFigure | null) => void;
}

function Board(props: IBoardProps) {
  const board = [];
  for (let i = 0; i < 10; i++) {
    const squareRows = [];
    for (let j = 0; j < 10; j++) {
      const squareNo = i * 10 + j;
      const backgroundColor = isOnDarkDiag(squareNo) ? "#693e3e" : "#ffd6ae";
      const figure = props.figureMap.get(squareNo) ?? null;
      squareRows.push(
        <Square
          key={squareNo}
          value={figure}
          onClick={() => props.onClick(squareNo, figure)}
          color={
            props.possibleMoves.some((move) => move.square_no === squareNo)
              ? "sandybrown"
              : backgroundColor
          }
        />
      );
    }
    board.push(
      <div key={i} className="board-row">
        {squareRows}
      </div>
    );
  }
  return <div>{board}</div>;
}

export default function Game() {
  const [figureMap, setFigureMap] = useState<Map<number, IFigure>>(
    getInitialFiguresState()
  );
  const [selectedFigureNo, setSelectedFigureNo] = useState<number | null>(null);
  const [whiteIsNext, setWhiteIsNext] = useState<boolean>(true);
  const [possibleMoves, setPossibleMoves] = useState<Move[]>([]);

  useEffect(() => {
    init();
  });

  useEffect(() => {
    const possibleMoves = selectedFigureNo
      ? possible_moves(selectedFigureNo, figureMap)
      : [];
    setPossibleMoves(possibleMoves);
  }, [figureMap, selectedFigureNo]);

  const makeMove = (
    move: Move,
    movedFigureNo: number,
    figureMap: Map<number, IFigure>
  ): [boolean, IFigure, Map<number, IFigure>] => {
    const movedFigure = figureMap.get(movedFigureNo);
    if (!movedFigure)
      return [
        false,
        { kind: "man", color: "white" },
        new Map<number, IFigure>(),
      ];
    const newFigureMap = new Map<number, IFigure>(figureMap);

    //Move selected figure
    newFigureMap.delete(movedFigureNo);
    newFigureMap.set(move.square_no, movedFigure);

    let isMultiCaptureScenario: boolean = false;
    if (move.is_capture && move.captured_figure_no) {
      //Delete captured figure
      newFigureMap.delete(move.captured_figure_no);
      //Check if multi-capture scenario isn't happening
      isMultiCaptureScenario = possible_moves(move.square_no, newFigureMap).some(
        (move) => move.is_capture === true
      );
    }
    return [isMultiCaptureScenario, movedFigure, newFigureMap];
  };

  const handleClick = (
    clickedSquareNo: number,
    clickedSqareFigure: IFigure | null
  ) => {
    let move = possibleMoves.find((move) => move.square_no === clickedSquareNo);
    if (
      figureMap.get(clickedSquareNo)?.color ===
      (whiteIsNext ? "white" : "black")
    ) {
      setSelectedFigureNo(
        selectedFigureNo && selectedFigureNo === clickedSquareNo
          ? null
          : clickedSquareNo
      );
    } else if (selectedFigureNo && move) {
      let [isMultiCaptureScenario, movedFigure, newFigureMap] = makeMove(
        move,
        selectedFigureNo,
        figureMap
      );
      if (isMultiCaptureScenario) {
        setSelectedFigureNo(clickedSquareNo);
      } else {
        setSelectedFigureNo(null);
        setWhiteIsNext(!whiteIsNext);
        if (becomesKing(move.square_no, movedFigure)) {
          newFigureMap.set(move.square_no, {
            kind: "king",
            color: movedFigure.color,
          });
        }
      }
      setFigureMap(newFigureMap);
    }
  };

  return (
    <div className="game">
      <div className="game-board">
        <div className="status">
          {"Next player: " + (whiteIsNext ? "White" : "Black")}
        </div>
        <Board
          figureMap={figureMap}
          possibleMoves={possibleMoves}
          onClick={handleClick}
        />
      </div>
    </div>
  );
}

const isOnDarkDiag = (i: number): boolean => {
  return [1, 3, 5, 7, 9].includes(Math.abs((i % 10) - Math.floor(i / 10)) % 11);
};

const becomesKing = (sqareNo: number, figure: IFigure): boolean => {
  if (figure.kind === "king") return false;
  return figure.color === "black"
    ? [...Array(10).keys()].map((i) => i + 90).includes(sqareNo)
    : [...Array(10).keys()].includes(sqareNo);
};

const getInitialFiguresState = (): Map<number, IFigure> => {
  const map = new Map<number, IFigure>();
  for (let i = 0; i < 100; ++i) {
    if (isOnDarkDiag(i)) {
      if (i < 40) map.set(i, { color: "black", kind: "man" });
      if (i >= 60) map.set(i, { color: "white", kind: "man" });
    }
  }
  return map;
};
