import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  template: '<router-outlet />',
})
export class AppComponent {
  constructor() {
    document.documentElement.dataset['platform'] = /Mac/i.test(navigator.userAgent)
      ? 'mac'
      : 'win';
  }
}
