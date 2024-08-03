import { Injectable } from '@angular/core';
import { RxStomp, IMessage } from '@stomp/rx-stomp';
import { Subject, Subscription, Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { BoardMessage } from './dto/board-message';
import { MoveMessage } from './dto/move-message';
import { MoveRequest } from './dto/move-request';
import { ChatRequest } from './dto/chat-request';
import { ChatMessage } from './dto/chat-message';
import { BoardEvent } from './dto/board-event';
import { ChatEvent } from './dto/chat-event';

@Injectable({
  providedIn: 'root'
})
export class WebsocketService {
  private apiUrl: String;
  rxStomp: RxStomp = new RxStomp();

  private boardSubject: Subject<BoardEvent> = new Subject<BoardEvent>();
  private boardQueue?: Subscription;
  private boardOOB?: Subscription;
  board$: Observable<BoardEvent> = this.boardSubject.asObservable();

  private moveSubject: Subject<MoveMessage> = new Subject<MoveMessage>();
  private moveQueue?: Subscription;
  move$: Observable<MoveMessage> = this.moveSubject.asObservable();

  private chatSubject: Subject<ChatEvent> = new Subject<ChatEvent>();
  private chatQueue?: Subscription;
  chat$: Observable<ChatEvent> = this.chatSubject.asObservable();
  id?: number;

  constructor() {
    this.apiUrl = environment.apiUrl.replace(/^http/, 'ws');
    if (!this.apiUrl.startsWith("ws")) {
      let frontendUrl = window.location.origin.replace(/^http/, 'ws');
      this.apiUrl = frontendUrl + environment.apiUrl;
    }
    console.log(this.apiUrl);
  }

  connect() {
    if (this.rxStomp.connected()) {
      return;
    }
    let url = this.apiUrl + "/play/websocket";
    let tokenFromStorage = localStorage.getItem("token");
    let token = tokenFromStorage == null ? "" : tokenFromStorage;

    this.rxStomp.configure({
      brokerURL: url,
      connectHeaders: {
        Token: token,
      },

      heartbeatIncoming: 0,
      heartbeatOutgoing: 20000,
      reconnectDelay: 500,
    });

    console.log("activating");
    this.rxStomp.activate();
  }

  makeMove(gameId: number, move: MoveRequest) {
    this.rxStomp.publish({ destination: `/app/move/${gameId}`, body: JSON.stringify(move) });
  }

  sendChat(msg: ChatRequest) {
    if(!this.id) {
      return;
    }
    this.rxStomp.publish({ destination: `/app/chat/${this.id}`, body: JSON.stringify(msg) });
  }

  subscribeGame(gameId: number) {
    this.id = gameId;
    this.unsubscribe();
    this.subscribeMoves(gameId);
    this.subscribeBoard(gameId);
    this.subscribeChat(gameId);
  }

  unsubscribe() {
    this.id = undefined;
    this.chatQueue?.unsubscribe();
    this.moveQueue?.unsubscribe();
    this.boardQueue?.unsubscribe();
    this.boardOOB?.unsubscribe();
  }

  disconnect() {
    this.rxStomp.deactivate();
  }

  subscribeMoves(gameId: number) {
    this.moveQueue = this.rxStomp
      .watch(`/topic/game/${gameId}`)
      .subscribe((message: IMessage) => {
        let move: MoveMessage = JSON.parse(message.body)
        this.moveSubject.next(move);
      });
  }

  subscribeBoard(gameId: number) {
    this.boardOOB = this.rxStomp
      .watch(`/app/board/${gameId}`)
      .subscribe((message: IMessage) => {
        let board: BoardMessage = JSON.parse(message.body)
        this.processBoard(board);
        this.boardOOB?.unsubscribe();
      });
    this.boardQueue = this.rxStomp
      .watch(`/topic/board/${gameId}`)
      .subscribe((message: IMessage) => {
        let board: BoardMessage = JSON.parse(message.body)
        this.processBoard(board);
      });
  }

  processBoard(message: BoardMessage) {
    const username = localStorage.getItem("username");
    let inverted = false;
    let color: "Black" | "White" | undefined = undefined;
    if(username) {
      if(message.username1 == username) {
        inverted = !message.firstUserStarts;
      } else if(message.username2 == username) {
        inverted = message.firstUserStarts;
      }
      color = inverted ? "Black" : "White";
    }

    this.boardSubject.next({board: message, inverted: inverted, color: color});
  }

  subscribeChat(gameId: number) {
    this.chatQueue = this.rxStomp
      .watch(`/topic/chat/${gameId}`)
      .subscribe((message: IMessage) => {
        let msg: ChatMessage = JSON.parse(message.body)
        this.processChat(msg);
      });
  }

  processChat(message: ChatMessage) {
    const username = localStorage.getItem("username");
    let self = username == message.player;
    this.chatSubject.next({message: message.message, player: message.player, self: self});
  }
}
