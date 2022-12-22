import React, { useEffect, useState } from 'react';
import './App.css';
import init, { add } from "rust-wasm-lib";

interface IFigure { 
  color: "black" | "white";
  type: "man" | "king";
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
      {props.value && <Figure color={props.value.color} type={props.value.type} />}
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
      const backgroundColor = 
      [1, 3, 5, 7, 9].includes(Math.abs(j-i)%11) ? "#693e3e" : "#ffd6ae";
      const squareNo = i*10+j;
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

export default class Game extends React.Component<{}, IGameState> {
  constructor(props : IGameState) {
    super(props);
    this.state = {
      squareValue: Array(100).fill(null),
      seletedSquare: null,
      whiteIsNext: true,
    };
    for (let i=0; i<30; ++i) {
      if ([1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11))
        this.state.squareValue[i] = {color:'black', type:'man'};
    }
    for (let i=99; i>=70; --i) {
      if ([1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11))
        this.state.squareValue[i] = {color:'white', type:'man'};
    }
  }

  handleClick = (i:number, figure:IFigure|null) => {
    const squares = this.state.squareValue.slice();
    if (squares[i] !== null && this.state.seletedSquare === null) {

    }

    this.setState({
      squareValue: squares,
      seletedSquare: i,
    });
  }

  render() {
    const winner = calculateWinner(this.state.squareValue);
    let status;
    if (winner)
      status = 'Winner: ' + winner;
    else
      status = 'Next player: ' + (this.state.whiteIsNext ? 'White' : 'Black');
    return (
      <div className="game">
        <div className="game-board">
        <div className="status">{status}</div>
          <Board 
          squareValue={this.state.squareValue}
          selectedSquare={this.state.seletedSquare}
          onClick={this.handleClick}
          />
        </div>
      </div>
    );
  }
}


function getPossibleMoves(square: any, figure: any){
}

function calculateWinner(squares: any[]){
  return null;
}

