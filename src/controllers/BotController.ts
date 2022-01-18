import { Context, Controller, Check } from "@Controller"
import { Bot, CreateBotSchema } from '@structures'
import config from '@config';

export class BotController extends Controller('/bots') {
  'USE /'() {
    // Not Implemented yet.
    return 501
  }

  'GET /'(ctx: Context): Promise<Bot[]> {
    return ctx.user.fetchBots()
  }

  'GET /:bot_id'(ctx: Context): Promise<Bot> {
    return Bot.findOne({ id: ctx.params.bot_id })
  }


  @Check(CreateBotSchema)
  async 'POST /'(ctx: Context): Promise<Bot> {
    const botCount = await Bot.count(`owner_id = ${ctx.user.id}`)

    if (botCount >= config.limits.user.bots) ctx.throw('MAXIMUM_BOTS')

    const bot = Bot.from({
      ...ctx.body,
      owner_id: ctx.user.id
    })

    await bot.save()

    return bot
  }

  async 'DELETE /:bot_id'(ctx: Context) {
    const bot = await Bot.findOne({ id: ctx.params.bot_id })
    await bot.delete()
  }
}
