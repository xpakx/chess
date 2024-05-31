import { Component, Input, OnInit } from '@angular/core';
import { Avatar } from '../dto/avatar';

@Component({
  selector: 'app-avatar',
  templateUrl: './avatar.component.html',
  styleUrls: ['./avatar.component.css']
})
export class AvatarComponent implements OnInit {
  @Input() avatar: Avatar = "avocado";

  constructor() { }

  ngOnInit(): void {
  }

}
