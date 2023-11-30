import { FC, PropsWithChildren } from 'react'
import 'twin.macro'
import { ConnectButton } from '../web3/ConnectButton'

export const Header: FC<PropsWithChildren> = ({ children }) => {
  return (
    <>
      <header tw="fixed z-10 flex w-full justify-end p-8">
        {/* Connect Wallet Button */}
        <ConnectButton />
      </header>
    </>
  )
}
