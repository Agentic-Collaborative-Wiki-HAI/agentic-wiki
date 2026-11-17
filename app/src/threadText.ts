// A standalone mention addresses the agent; an email address or a longer handle does not.
export const mentionsAgent = (text: string) => /(^|\s)@agent(?=$|[\s.,!?:;])/i.test(text)
export function messageParts(text: string) {
  return text
    .split(/((?:^|\s)@agent(?=$|[\s.,!?:;]))/gi)
    .filter(Boolean)
    .map((value) => ({ text: value, mention: mentionsAgent(value) }))
}
