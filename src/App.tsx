import React, { useEffect, useState } from 'react';
import './App.css';
import init, { add } from "rust-wasm-lib";
import { Console } from 'console';

interface ISquare {
  onClick: React.MouseEventHandler<HTMLButtonElement>; 
  value: string;
  color: string;
}

function Square(props: ISquare) 
{
  const [isShowingMoves, setisShowingMoves] = useState(false);

  return (
    <button className="square" style={{ backgroundColor: props.color}} onClick={props.onClick}>
      {props.value}
    </button>
  );
}

interface IBoardState {
  squares: any[];
  whiteIsNext: boolean;
}

class Board extends React.Component<{}, IBoardState> {
  constructor(props : IBoardState) {
    super(props);
    this.state = {
      squares: Array(100).fill(null),
      whiteIsNext: true,
    };
    for (let i=0; i<30; ++i) {
      if ([1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11))
        this.state.squares[i] = <div className="circle" style={{ backgroundColor: "black"}}/>;
    }
    for (let i=99; i>=70; --i) {
      if ([1, 3, 5, 7, 9].includes(Math.abs(i%10-Math.floor(i/10))%11))
        this.state.squares[i] = <div className="circle" style={{ backgroundColor: "white"}}/>;
    }
  }


  handleClick(i:number) {
    const squares = this.state.squares.slice();

    if (squares[i] !== null) {
      squares[i+1] = <div className="square" style={{ backgroundColor: 'sandybrown'}} />
    }

    this.setState({
      squares: squares,
      whiteIsNext: !this.state.whiteIsNext,
    });
  }

  render() {
    const winner = calculateWinner(this.state.squares);
    let status;
    if (winner) {
      status = 'Winner: ' + winner;
    } else {
      status = 'Next player: ' + (this.state.whiteIsNext ? 'White' : 'Black');
    }

    const board = [];
    for(let i = 0; i < 10; i++){
      const squareRows = [];
      for(let j = 0; j < 10; j++){
        const backgroundColor = [1, 3, 5, 7, 9].includes(Math.abs(j-i)%11) ? "#693e3e" : "#ffd6ae";
        squareRows.push(
          <Square
          value={this.state.squares[i*10+j]}
          onClick={() => this.handleClick(i*10+j)}
          color={backgroundColor}
        />
        );
      }
      board.push(<div className="board-row">{squareRows}</div>)
    }

    return (
      <div>
        <div className="status">{status}</div>
        <div>
          {board}
        </div>
      </div>
    );
  }
}

class Game extends React.Component {
  render() {
    return (
      <div className="game">
        <div className="game-board">
          <Board />
        </div>
        <div className="game-info">
          <div>{/* status */}</div>
          <ol>{/* TODO */}</ol>
        </div>
      </div>
    );
  }
}

export default Game;

function getPossibleMoves(square: any, figure: any){
}

function calculateWinner(squares: string[]){
  return null;
}

