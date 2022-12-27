import React, { useEffect, useState } from 'react';
import './App.css';
import init, {possible_moves} from 'rust-wasm-lib';


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
  squareValue: (IFigure | null)[];
  selectedSquare: number | null;
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
      const squareValue = props.squareValue[squareNo];
      squareRows.push(
        <Square
        value={props.squareValue[squareNo]}
        onClick={() => props.onClick(squareNo, squareValue)}
        color={props.selectedSquare === squareNo ? 'sandybrown' : backgroundColor}
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

interface IGameState {
  squareValue: (IFigure | null)[];
  seletedSquare: number | null;
  whiteIsNext: boolean;
}

export default function Game() {
  const [squareValue, setSqareValue] = useState<(IFigure | null)[]>(
    Array(100).fill(null)
      .map((val:(IFigure | null), i:number) => {
          if (isOnDarkDiag(i)) {
            if (i < 30)
              return {color:'black', kind:'man'};
            if (i >= 70)
              return {color:'white', kind:'man'};
          }
          return null;
      })
  );
  const [seletedSquare, setSeletedSquare] = useState<number | null>(null);
  const [whiteIsNext, setWhiteIsNext] = useState<boolean>(true);
  const [possibleMoves, setPossibleMoves] = useState<number[]|null>(null);

  useEffect(() => {
    init().then(() => {
      setPossibleMoves(seletedSquare && possible_moves(seletedSquare, squareValue[seletedSquare], squareValue))
    })
  });
  const handleClick = (i:number, figure:IFigure|null) => {
    const squares = squareValue.slice();
    let newSelectedSquare = null;
    if (squares[i] !== null) {
      if (seletedSquare && seletedSquare === i)
        newSelectedSquare = null;
      else 
        newSelectedSquare = i;
      
      console.log(`i:${i}, figure:${figure?.color} kind:${figure?.kind}`)
    }

    setSqareValue(squares);
    setSeletedSquare(newSelectedSquare);
  }


  
  return (
    <div className="game">
      <div className="game-board">
      <div className="status">{'Next player: ' + (whiteIsNext ? 'White' : 'Black')}</div>
        <Board 
        squareValue={squareValue}
        selectedSquare={seletedSquare}
        onClick={handleClick}
        />
      </div>
    </div>
  );
}

const isOnDarkDiag = (i:number) : boolean => {
  return [1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11)
}

function calculateWinner(squares: any[]){
  return null;
}

