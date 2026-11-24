import { Component, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { AppEnv } from './app-env';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  template: '<router-outlet />',
})
export class AppComponent {
  /** 确保各窗启动即初始化平台 / 主题同步 */
  private readonly _env = inject(AppEnv);
}
