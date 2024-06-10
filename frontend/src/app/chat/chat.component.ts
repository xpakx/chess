import { Component, OnInit } from '@angular/core';
import { Subscription } from 'rxjs';
import { ChatMessage } from '../board/dto/chat-message';
import { WebsocketService } from '../board/websocket.service';
import { ChatEvent } from '../board/dto/chat-event';
import { FormGroup, FormBuilder } from '@angular/forms';

@Component({
  selector: 'app-chat',
  templateUrl: './chat.component.html',
  styleUrls: ['./chat.component.css']
})
export class ChatComponent implements OnInit {
  chat: ChatEvent[] = [];
  private chatSub?: Subscription;
  chatForm: FormGroup;

  constructor(private formBuilder: FormBuilder, private websocket: WebsocketService) {
    this.chatForm = this.formBuilder.group({ message: [''] });
  }

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

  sendMessage() {
    if (this.chatForm.invalid) {
      return;
    }
    let message = this.chatForm.value.message;
    this.websocket.sendChat(message);
  }
}

