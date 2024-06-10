import { Component, OnInit } from '@angular/core';
import { Subscription } from 'rxjs';
import { ChatMessage } from '../board/dto/chat-message';
import { WebsocketService } from '../board/websocket.service';
import { ChatEvent } from '../board/dto/chat-event';

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
  styleUrls: ['./chat.component.css']
})
export class ChatComponent implements OnInit {
  chat: ChatEvent[] = [];
  private chatSub?: Subscription;

  constructor(private websocket: WebsocketService) { }

  ngOnInit(): void {
    this.chatSub = this.websocket.chat$
    .subscribe((event: ChatEvent) => this.onChat(event));
  }

  onChat(message: ChatEvent) {
    this.chat.push(message);
  }

  ngOnDestroy(): void {
    this.chatSub?.unsubscribe();
  }
}
