import { useCallback, useContext } from "react"
import { Theme } from "styled-rn"

import { PublishingPartnerAppearance } from "$relay/PuzzmoCurrentUserFragment.graphql"
import { PublishingPartnerNavBackground, PublishingPartnerNavForeground } from "$relay/PuzzmoQuery.graphql"

import { AppContext } from "../../AppContext"
import { createEventEmitter, makeUseEmitterSubscription } from "../lib/createEmitter"

type MessagesRecieved = { MSG_TO_APP: { test: string }; AUTH_MODEL: { type: "login" | "signup" } }

// This is immutable! Do not _remove_ or _change_ the strings in these fields.
// The native app needs these to operate.

type AppSpecificPartner = {
  backURL: string
  navHeight: number
  logoLongBlack: string
  logoLongWhite: string
  logoHeight: number
  logoWidth: number
  ourLogoOffsets: readonly number[] // [number, number]
  theirLogoOffsets: readonly number[] //[number, number]
  appearance: PublishingPartnerAppearance
  navBG: PublishingPartnerNavBackground
  navFG: PublishingPartnerNavForeground
  ourLogoFG: PublishingPartnerNavBackground
  ourLogoFGOverride: string | null | undefined
}

type MessagesSent =
  | {
      type: "app-context"
      userStateID: string
      userID: string | undefined
      partnerID: string | undefined
      partner: AppSpecificPartner | null | undefined
      theme: Theme
    }
  | {
      type: "start-sub"
      successURL: string
      failURL: string
      accountSlug: string
      partnerSlug: string
      impactClickID: string
    }
  | {
      type: "start-gift"
      successURL: string
      failURL: string
      accountSlug: string
      partnerSlug: string
      impactClickID: string
    }

// | { type: "manage-billing"; accountSlug: string }

const nativeEventsEmitter = createEventEmitter<MessagesRecieved>()
// @ts-expect-error - because of globalThis
globalThis.nativeEventsEmitter = nativeEventsEmitter

export const useSubscribeToNativeMessage = makeUseEmitterSubscription<MessagesRecieved>(nativeEventsEmitter)

export const useSendNativeMessage = () => {
  const { appRuntime } = useContext(AppContext)

  return useCallback(
    (message: MessagesSent) => {
      if (appRuntime !== "native") return
      if (!("webkit" in window)) throw new Error("No webkit on window")
      if ((window as any)?.webkit?.messageHandlers?.app === undefined) throw new Error("No native handler")
      ;(window as any).webkit.messageHandlers.app.postMessage(message)
    },
    [appRuntime],
  )
}
