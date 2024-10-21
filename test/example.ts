// https://playground.oxc.rs/#eNrNVstu2zoQ/RVGuAsb8GOvXDc3z+YCebhxgi7iwKCkkUyEIlWSsuOm/vcORdqSHbntsoYBiZzDOfPijN6DOAgDlhdSGfJOSg3nlPOIxq+9aiGFgTdD1iRVMifTQAGNzTSYiu2RxznkUAO0WXFI+kpYUAM2LiPO9JyJbEyVEaBOiwKooiJuHP5HAaer4bj8/j2X56VSIMyTBnWlaJbj+yBTtJh/4zsGfNB8Rxdn6ECmZCmSXqv8Sipw8gPkX0pQqyZdgxAt/xCXwWCI/1qyY2GMUTNwuUAPLnNmDKgeyekroGt+PSkjHStWGCbFjlLOoqE/7pDOFrMqgNyC1jQD/QAxgwUkZIRct5PPs8f72el4HOLKgDYh0Uah92R9TE6fHq9nt/cXlzeVFLWEyMNlxjBf5IfNH8tEWeBiTdaWaTjEDDNN7D/PS0MjDkfkQhIhDZkpyOUCZkQqMovnVGT4bubgGfGIsEsNJGXAEz3w+oAIatgCCC0KIgAS7WFGElmAQncR6t3EoE4KdDFlsU+gdXQqCLFl+vSArjg6uyXo4hpYNkevRZlHGC/cRP/kjRTZGccDTfRG8HXODEZiV9Cu5ytLzLy5KUt1g/v3aarB6JBgshIp+MpDnl8IuvzsFj2/+WIPosfs90fbTmLU/NUJf3WvfDzOPregdu6IB14dANaXpeHvIfCu1i34fgFKsWQbZKw1UXKOD0RCygQgviq4ndKe4JUhI6vph0u5/W3KFsPQj+vr5oTYtNTEYAH9f9FM6EZU7+5SO0Th3PgjUNhWmG1eeaNtmwxdt3R76G2rW9qgsr4uo9opXcYxRmSv1u0vpYy3bNM4xhSYCS+zfZE3v02EDQu7+zln8et+9H5tbMbSRgr+CmurToMGby3NqcCK6keMc4sLjtto3UksKm18i6q6tvbNFxvPx17+734f/tTpVuz/Gd2HN6wQ08fixxbZJxHEFMuQyJRkXEaU2946FfX7oJ21ZdcaitrthHH2ol4/RiJ4lHfVCW8aajg8b1rsb6FDl1r4QCT7RJ0uGX1yheJw77ZfPWCgWfWdMGp8V3TqgVnpJ9gCTalE8zuk41LayR1DuNMbaq6qIlLSaXAdjUaYd+fLNOh63U3wUWcaLCF6ZVi9dlYtmUjkstvF26rkEifTklzazCHsThKHJDihHQ51NpV13C6hmlCx6p4MHP5k4C2/piLhoPTJwE69ERq3bRIHCP2QxLlqDzbojve4PNU+kyUaFFIbH7JNDL2edc89n+uYvVRbKMd7EPQCGYTvgSqFfeiVMPQtCI0qoRfgHTKbdx3jzN4uVnkk+WZlcBDpVKo8CFPKNax7Ad5o7MOoEd+tls37FrrZiGUCGVhuXORMYJ+tRcIoya+4XFrTcLZEUqMJjmO9/gm/6s/z

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
