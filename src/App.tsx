import React, { useEffect, useState } from 'react';
import './App.css';
import init, {possible_moves} from './pkg';


interface IFigure { 
  color: "black" | "white";
  kind: "man" | "king";
}

function Figure(props: IFigure)
{
  return (
    <div className="circle" style={{ backgroundColor: props.color}}/>
  );
}

interface ISquareProps {
  onClick: React.MouseEventHandler<HTMLButtonElement>; 
  value: IFigure | null;
  color: string;
}

function Square(props: ISquareProps) 
{
  return (
    <button 
    className="square"
    style={{ backgroundColor: props.color}} 
    onClick={props.onClick}>
      {props.value && <Figure color={props.value.color} kind={props.value.kind} />}
    </button>
  );
}

interface IBoardProps {
  figure: Map<number, IFigure>;
  possibleMoves: number[] | null;
  onClick: (squareNo:number, squareValue:IFigure|null) => void;
}

function Board(props: IBoardProps) {
  const board = [];
  for(let i = 0; i < 10; i++){
    const squareRows = [];
    for(let j = 0; j < 10; j++){
      const squareNo = i*10+j;
      const backgroundColor = 
      isOnDarkDiag(squareNo) ? "#693e3e" : "#ffd6ae";
      const figure = props.figure.get(squareNo) ?? null;
      squareRows.push(
        <Square
        value={figure}
        onClick={() => props.onClick(squareNo, figure)}
        color={props.possibleMoves && props.possibleMoves[squareNo] ? 'sandybrown' : backgroundColor}
      />
      );
    }
    board.push(<div className="board-row">{squareRows}</div>)
  }

  return (
    <div>
      {board}
    </div>
  );
}

export default function Game() {
  const [figureMap, setFigureMap] = useState<Map<number, IFigure>>(
    getInitialFiguresState()
    );
  const [seletedSquare, setSeletedSquare] = useState<number | null>(null);
  const [whiteIsNext, setWhiteIsNext] = useState<boolean>(true);
  const [possibleMoves, setPossibleMoves] = useState<number[]|null>(null);

  useEffect(() => {
    init().then(() => {
      const possibleMoves = seletedSquare && possible_moves(seletedSquare, figureMap);
      setPossibleMoves(possibleMoves)
    })
  });
  const handleClick = (i:number, figure:IFigure|null) => {
    const newFigureMap = new Map<number, IFigure>(figureMap);
    let newSelectedSquare = null;
    if (figureMap.get(i)) {
      newSelectedSquare = (seletedSquare && seletedSquare === i) ? null : i;
      console.log(seletedSquare);
      console.log(seletedSquare && figureMap.get(seletedSquare));
      console.log(possibleMoves);
    }

    setSeletedSquare(newSelectedSquare);
    setFigureMap(newFigureMap);
  }


  
  return (
    <div className="game">
      <div className="game-board">
      <div className="status">{'Next player: ' + (whiteIsNext ? 'White' : 'Black')}</div>
        <Board 
        figure={figureMap}
        possibleMoves={possibleMoves}
        onClick={handleClick}
        />
      </div>
    </div>
  );
}

const isOnDarkDiag = (i:number) : boolean => {
  return [1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11)
}

const getInitialFiguresState = () : Map<number, IFigure> => {
  const map = new Map<number, IFigure>();
  for (let i=0; i<100; ++i) {
    if (isOnDarkDiag(i)) {
      if (i < 30)
        map.set(i, {color:'black', kind:'man'});
      if (i >= 70)
        map.set(i, {color:'white', kind:'man'});
    }
  }
  return map;
} 


