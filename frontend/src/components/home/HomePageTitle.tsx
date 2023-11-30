import Image from 'next/image'
import Link from 'next/link'
import kudosLogo from 'public/brand/kudos-squid.png'
import githubIcon from 'public/icons/github-button.svg'
import { FC } from 'react'
import 'twin.macro'
import tw, { styled } from 'twin.macro'

const StyledIconLink = styled(Link)(() => [
  tw`opacity-90 transition-all hover:(-translate-y-0.5 opacity-100)`,
])

export const HomePageTitle: FC = () => {
  const title = 'kudos ink!'
  const desc =
    'An automated contribution reward service using custom smart contracts and Github workflows.'
  const githubHref = 'https://github.com/kudos-ink'
  const deployHref = 'https://github.com/kudos-ink/demo'
  // const sponsorHref = 'mailto:hello@scio.xyz'
  // const telegramHref = 'https://t.me/inkathon'

  return (
    <>
      <div tw="flex flex-col items-center text-center font-mono">
        {/* Logo & Title */}
        <Link
          href="/"
          className="group"
          tw="flex cursor-pointer items-center gap-4 rounded-3xl py-1.5 px-3.5 transition-all hover:bg-sky-900"
        >
          <Image src={kudosLogo} priority width={60} alt="Kudos Logo" />
          <h1 tw="font-black text-[2.5rem]">{title}</h1>
        </Link>

        {/* Tagline & Links */}
        <p tw="mt-2 text-sky-600 text-sm">
          By{' '}
          <a
            href="https://github.com/ipapandinas"
            target="_blank"
            tw="font-semibold text-sky-300 hover:text-white"
          >
            Igor Papandinas
          </a>{' '}
          x{' '}
          <a
            href="https://linktr.ee/leapalazzolo"
            target="_blank"
            tw="font-semibold text-sky-300 hover:text-white"
          >
            Leandro Palazzolo
          </a>{' '}
          x{' '}
          <a
            href="https://www.linkedin.com/in/connor-campbell-4376b1181/"
            target="_blank"
            tw="font-semibold text-sky-300 hover:text-white"
          >
            Connor Campbell
          </a>
        </p>
        <p tw="mt-4 mb-6 text-white">{desc}</p>

        {/* Github & Vercel Buttons */}
        <div tw="flex space-x-2">
          <StyledIconLink href={githubHref} target="_blank">
            <Image src={githubIcon} priority height={32} alt="Github Repository" />
          </StyledIconLink>
          {/* <StyledIconLink href={deployHref} target="_blank">
            <Image src={vercelIcon} priority height={32} alt="Deploy with Vercel" />
          </StyledIconLink>
          <StyledIconLink href={telegramHref} target="_blank">
            <Image src={telegramIcon} priority height={32} alt="Telegram Group" />
          </StyledIconLink>
          <StyledIconLink href={sponsorHref} target="_blank">
            <Image src={sponsorIcon} priority height={32} alt="Sponsor the Project" />
          </StyledIconLink> */}
        </div>

        <div tw="my-14 w-14 bg-sky-800 h-[2px]" />
      </div>
    </>
  )
}
