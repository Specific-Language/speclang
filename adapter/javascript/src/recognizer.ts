import { Context } from "./context"
import { LanguageContext } from "./language"

export const DEFAULT_OPTIONS = {
  verbose: false,
}

export class Recognizer {
  options: typeof DEFAULT_OPTIONS
  language: Context
  context: Context

  constructor(options: typeof DEFAULT_OPTIONS = DEFAULT_OPTIONS) {
    this.options = options
    this.language = LanguageContext()
    this.context = new Context()
  }
}
