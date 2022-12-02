import { Interval, io, ctx, ActionCtx } from '@interval/sdk';
import 'dotenv/config'; // loads environment variables from .env

import * as wasm from '../wasm/pkg';

global.log = console.log;

const actions = {};

const COMPLETED_DAYS = 2;

for (let i = 1; i <= COMPLETED_DAYS; i++) {
  actions[`day_${i}`] = async () => {
    const input = await io.input.text('Input', {
      multiline: true,
    });

    global.ctx_log = (s: string) => {
      ctx.log(s).catch(err => {
        console.error('error', err);
      });
    };

    const [part1, part2] = wasm[`day_${i}`](input);

    return {
      part1,
      part2,
    }
  }
}

const interval = new Interval({
  apiKey: process.env.INTERVAL_KEY,
  actions,
});

interval.listen();
