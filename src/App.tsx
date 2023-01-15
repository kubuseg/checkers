import React, { useCallback, useEffect, useState } from "react";
import "./App.scss";
import init, {
  Move,
  possible_moves,
  get_winner,
  forced_moves,
  get_best_move,
} from "./pkg/rust_wasm_lib";
import blackCrown from "./blackCrown.svg";
import whiteCrown from "./whiteCrown.svg";
import { Color } from "./pkg/rust_wasm_lib";
import lodash from "lodash";

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
  const [winner, setWinner] = useState<Color | null>(null);
  const [isPlayerMode, setIsPlayerMode] = useState<boolean>(false);
  const [selectedColor, setSelectedColor] = useState<Color>(Color.White);

  useEffect(() => {
    init().then(() => {
      //Get possible for selectedFigure
      const possibleMoves = selectedFigureNo
        ? possible_moves(selectedFigureNo, figureMap)
        : [];
      const forcedMoves = selectedFigureNo
        ? forced_moves(whiteIsNext ? Color.White : Color.Black, figureMap)
        : [];
      if (forcedMoves.length) {
        setPossibleMoves(
          possibleMoves.filter((move) =>
            forcedMoves.some((fMove) => lodash.isEqual(fMove, move))
          )
        );
      } else setPossibleMoves(possibleMoves);
    });
  }, [figureMap, selectedFigureNo, whiteIsNext]);

  const makeMove = useCallback(
    (move: Move) => {
      const newFigureMap = figureMap;

      //Move selected figure
      newFigureMap.delete(move.moved_figure_no);
      newFigureMap.set(move.square_no, move.moved_figure);

      let isMultiCaptureScenario: boolean = false;
      if (move.captured_figure_no) {
        //Delete captured figure
        newFigureMap.delete(move.captured_figure_no);
        //Check if multi-capture scenario is happening
        isMultiCaptureScenario = possible_moves(
          move.square_no,
          newFigureMap
        ).some((move) => move.captured_figure_no);
      }

      if (isMultiCaptureScenario) {
        setSelectedFigureNo(move.square_no);
      } else {
        setSelectedFigureNo(null);
        setWhiteIsNext(!whiteIsNext);
        setPossibleMoves([]);
        //If there is a multi-capture  scenario figure doesn't become king
        if (becomesKing(move.square_no, move.moved_figure)) {
          newFigureMap.set(move.square_no, {
            kind: "king",
            color: move.moved_figure.color,
          });
        }
      }
      setFigureMap(newFigureMap);
      setWinner(get_winner(newFigureMap));
    },
    [figureMap, whiteIsNext]
  );

  useEffect(() => {
    init().then(() => {
      //Bot move
      if (!isPlayerMode && !isPlayerTurn(whiteIsNext, selectedColor)) {
        const bestMove = get_best_move(
          selectedColor === Color.White ? Color.Black : Color.White,
          figureMap
        );
        bestMove.forEach((move) => makeMove(move));
      }
    });
  }, [figureMap, makeMove, whiteIsNext, isPlayerMode, selectedColor]);

  const handleClick = (
    clickedSquareNo: number,
    clickedSqareFigure: IFigure | null
  ) => {
    if (
      winner ||
      (!isPlayerMode && !isPlayerTurn(whiteIsNext, selectedColor))
    ) {
      return;
    }
    if (
      figureMap.get(clickedSquareNo)?.color ===
      (whiteIsNext ? "white" : "black")
    ) {
      setSelectedFigureNo(
        selectedFigureNo && selectedFigureNo === clickedSquareNo
          ? null
          : clickedSquareNo
      );
    }

    let move = possibleMoves.find((move) => move.square_no === clickedSquareNo);
    if (selectedFigureNo && move) {
      makeMove(move);
    }
  };

  const handleBotModeClick = () => {
    if (isPlayerMode) {
      setFigureMap(getInitialFiguresState());
      setSelectedFigureNo(null);
      setIsPlayerMode(false);
      setWhiteIsNext(true);
    }
  };

  const handlePlayerModeClick = () => {
    if (!isPlayerMode) {
      setFigureMap(getInitialFiguresState());
      setSelectedFigureNo(null);
      setIsPlayerMode(true);
      setWhiteIsNext(true);
    }
  };

  const handleResetGameClick = () => {
    setFigureMap(getInitialFiguresState());
    setSelectedFigureNo(null);
    setWhiteIsNext(true);
  };

  const onColorRadioButtonChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    setSelectedColor(strToColor(event.target.value));
    setFigureMap(getInitialFiguresState());
    setSelectedFigureNo(null);
    setWhiteIsNext(true);
  };

  return (
    <div className="game">
      <div onChange={onColorRadioButtonChange} className="color-select">
        <input
          type="radio"
          value="white"
          name="color"
          checked={selectedColor === Color.White}
          className="radio-button"
        />
        White
        <input
          type="radio"
          value="black"
          name="color"
          checked={selectedColor === Color.Black}
          className="radio-button"
        />
        Black
      </div>
      <div className="game-board">
        <div
          className="status"
          style={{ color: winner ? colorToStr(winner) : (whiteIsNext ? "white" : "black") }}
        >
          {winner
            ? `The winner is ${winner}!`
            : "Next player: " + (whiteIsNext ? "White" : "Black")}
        </div>
        <Board
          figureMap={figureMap}
          possibleMoves={possibleMoves}
          onClick={handleClick}
        />
      </div>
      <div className="container_row">
        <button className="button color1" onClick={handleBotModeClick}>
          Play with bot
        </button>
        <button className="button color2" onClick={handlePlayerModeClick}>
          Play with Player
        </button>
        <button className="button color3" onClick={handleResetGameClick}>
          Reset game
        </button>
      </div>
    </div>
  );
}

const isOnDarkDiag = (i: number): boolean => {
  return [1, 3, 5, 7, 9].includes(Math.abs((i % 10) - Math.floor(i / 10)) % 11);
};

const isPlayerTurn = (whiteIsNext: boolean, selectedColor: Color): boolean => {
  return selectedColor === (whiteIsNext ? Color.White : Color.Black);
};

const strToColor = (color: string) => {
  return color === "white" ? Color.White : Color.Black;
};

const colorToStr = (color: Color) => {
  return color === Color.White ? "white" : "black";
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
