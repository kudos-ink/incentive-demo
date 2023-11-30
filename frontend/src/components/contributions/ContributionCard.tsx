import { Button, Card, CardBody, CardFooter, Divider, Heading, Text } from '@chakra-ui/react'
import { useInkathon } from '@scio-labs/use-inkathon'
import Link from 'next/link'
import { FC } from 'react'
import { FiExternalLink } from 'react-icons/fi'

export interface Contribution {
  organization: string
  project: string
  projectUrl: string
  issueTitle: string
  issueUrl: string
}

export interface ContributionCardProps extends Contribution {}
export const ContributionCard: FC<ContributionCardProps> = ({
  organization,
  project,
  projectUrl,
  issueTitle,
  issueUrl,
}) => {
  const { activeAccount } = useInkathon()
  const title = `${organization} / ${project}`
  const issueURL = new URL(issueUrl, 'https://github.com')

  if (activeAccount) {
    issueURL.searchParams.set('address', activeAccount?.address)
  }

  return (
    <Card variant="outline" bgColor="whiteAlpha.100" w="sm">
      <CardBody display="flex" flexDirection="column">
        <Link href={projectUrl} target="_blank" title={title}>
          <Text tw="underline">{title}</Text>
        </Link>
        <Divider />
        <Text mt={4}>Task</Text>
        <Heading size="md">{issueTitle}</Heading>
      </CardBody>
      <CardFooter>
        <Link href={issueURL} target="_blank" title={issueTitle} style={{ width: '100%' }}>
          <Button flex="1" rightIcon={<FiExternalLink />} w="100%">
            Contribute
          </Button>
        </Link>
      </CardFooter>
    </Card>
  )
}
