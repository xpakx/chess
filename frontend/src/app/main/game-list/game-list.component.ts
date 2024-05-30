import { HttpErrorResponse } from '@angular/common/http';
import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { ToastService } from 'src/app/elements/toast.service';
import { Game } from '../dto/game';
import { GameManagementService } from '../game-management.service';
import { ListType } from '../dto/list-type';

@Component({
  selector: 'app-game-list',
  templateUrl: './game-list.component.html',
  styleUrls: ['./game-list.component.css']
})
export class GameListComponent implements OnInit {
  _view: ListType = "none";
  @Output() openGame: EventEmitter<number> = new EventEmitter<number>();

  games: Game[] = [
    {
      currentState: [
        ["BlackRook", "BlackKnight", "BlackBishop", "BlackQueen", "BlackKing", "BlackBishop", "BlackKnight", "BlackRook"],
        ["BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn"],
        ["WhiteRook", "WhiteKnight", "WhiteBishop", "WhiteQueen", "WhiteKing", "WhiteBishop", "WhiteKnight", "WhiteRook"],
      ],
      id: 1,
      invitation: "Accepted",
      gameType: "AI",
      aiType: "Random",
      gameStatus: "NotFinished",
      lastMoveRow: 0,
      lastMoveColumn: 0,
      userStarts: false,
      userTurn: false,
      invert: false,
      user: "Test",
      user_id: 1,
      opponent: "AI",
      opponent_id: 2,

    },
    {
      currentState: [
        ["BlackRook", "BlackKnight", "BlackBishop", "BlackQueen", "BlackKing", "BlackBishop", "BlackKnight", "BlackRook"],
        ["BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
        ["WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn"],
        ["WhiteRook", "WhiteKnight", "WhiteBishop", "WhiteQueen", "WhiteKing", "WhiteBishop", "WhiteKnight", "WhiteRook"],
      ],
      id: 2,
      invitation: "Accepted",
      gameType: "AI",
      aiType: "Random",
      gameStatus: "NotFinished",
      lastMoveRow: 0,
      lastMoveColumn: 0,
      userStarts: false,
      userTurn: false,
      invert: true,
      user: "Test",
      user_id: 1,
      opponent: "AI",
      opponent_id: 2,

    }
  ];
  activeView: boolean = false;
  requestsView: boolean = false;

  constructor(private gameService: GameManagementService, private toast: ToastService) { }

  @Input() set view(value: ListType) {
    this._view = value;
    if (value == "archive") {
      this.getArchive();
    } else if (value == "games") {
      this.getGames();
    } else if (value == "requests") {
      this.getRequests();
    } else {
      this.games = [];
      this.activeView = false;
      this.requestsView = false;
    }
  }

  ngOnInit(): void {
  }

  getRequests() {
    this.gameService.getGameRequests()
      .subscribe({
        next: (games: Game[]) => this.onRequests(games),
        error: (err: HttpErrorResponse) => this.onError(err)
      });
  }

  getGames() {
    this.gameService.getActiveGames()
      .subscribe({
        next: (games: Game[]) => this.onGames(games),
        error: (err: HttpErrorResponse) => this.onError(err)
      });
  }

  getArchive() {
    this.gameService.getFinishedGames()
      .subscribe({
        next: (games: Game[]) => this.onArchive(games),
        error: (err: HttpErrorResponse) => this.onError(err)
      });

  }

  onRequests(games: Game[]) {
    this.games = games;
    this.activeView = false;
    this.requestsView = true;
  }

  onArchive(games: Game[]) {
    this.games = games;
    this.activeView = false;
    this.requestsView = false;
  }

  onGames(games: Game[]) {
    this.games = games;
    this.activeView = true;
    this.requestsView = false;
  }

  accept(gameId: number) {
    this.gameService.acceptRequest(gameId, { status: "Accepted" })
      .subscribe({
        next: (value: Boolean) => this.onAccept(gameId),
        error: (err: HttpErrorResponse) => this.onError(err)
      });
  }

  onAccept(gameId: number) {
    this.open(gameId);
    this.toast.createToast({message: "Request accepted", id: `rejection-${gameId}`, type: "info"});
  }

  reject(gameId: number) {
    this.gameService.acceptRequest(gameId, {status: "Rejected"})
      .subscribe({
        next: (value: Boolean) => this.onReject(gameId),
        error: (err: HttpErrorResponse) => this.onError(err)
      });

  }

  onReject(gameId: number) {
    this.games = this.games.filter((game) => game.id != gameId);
    this.toast.createToast({message: "Request rejected", id: `rejection-${gameId}`, type: "info"});
  }

  onError(err: HttpErrorResponse) {
    this.toast.createToast({message: err.error, id: `error-${new Date().toTimeString}`, type: "error"});
  }

  open(gameId: number) {
    this.openGame.emit(gameId);
  }
}
