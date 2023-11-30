import { ContractIds } from '@/deployments/deployments'
import { truncateHash } from '@/utils/truncateHash'
import {
  AlertStatus,
  Button,
  Card,
  FormControl,
  FormLabel,
  Input,
  Stack,
  useToast,
} from '@chakra-ui/react'
import { encodeAddress } from '@polkadot/util-crypto'
import {
  contractQuery,
  decodeOutput,
  useInkathon,
  useRegisteredContract,
} from '@scio-labs/use-inkathon'
import { useRouter } from 'next/router'
import { FC, useEffect, useState } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import 'twin.macro'

type Inputs = { contributionId: number }
type ToastData = {
  title: string
  description: string
  status: AlertStatus
  duration: number
}

export const ContributionStatus: FC = () => {
  const router = useRouter()
  const defaultContributionId = router.query.contributionId as string
  const { api, activeAccount, activeSigner } = useInkathon()
  const { contract, address: contractAddress } = useRegisteredContract(ContractIds.Demo)
  const [isReady, setIsReady] = useState<boolean>(false)
  const [updateIsLoading, setUpdateIsLoading] = useState<boolean>()
  const { register, reset, handleSubmit, formState, trigger } = useForm<Inputs>()
  const chakraToast = useToast()

  const issueToast = ({ title, description, status, duration }: ToastData) => {
    return chakraToast({
      title,
      description,
      status,
      duration,
      isClosable: true,
    })
  }

  const checkContributionStatus = async ({ contributionId }: Inputs) => {
    if (!activeAccount || !contract || !activeSigner || !api) {
      toast.error('Wallet not connected. Try again…')
      return
    }

    setUpdateIsLoading(true)
    try {
      const result = await contractQuery(
        api,
        activeAccount.address,
        contract,
        'getContributor',
        {},
        [contributionId],
      )
      const { output, isError, decodedOutput } = decodeOutput(result, contract, 'getContributor')
      const isSelfContributor = output === activeAccount.address
      if (!isError) {
        if (!output) {
          issueToast({
            title: `Issue #${contributionId} not yet approved`,
            description: `No contributor has been found for this issue number`,
            status: 'warning',
            duration: 9000,
          })
        } else {
          issueToast({
            title: `Issue #${contributionId} approved`,
            description: `${
              isSelfContributor
                ? 'Kudos for contributing!'
                : `${truncateHash(
                    encodeAddress(output, 42),
                    8,
                  )} has made a contribution which has been approved`
            }`,
            status: 'success',
            duration: 9000,
          })
        }
      } else {
        issueToast({
          title: `Something went wrong`,
          description: decodedOutput,
          status: 'error',
          duration: 9000,
        })
      }
      reset()
    } catch (e) {
      console.error(e)
    } finally {
      setUpdateIsLoading(false)
    }
  }

  useEffect(() => {
    if (!isReady && activeAccount && contract && activeSigner && api) {
      const contributionId = parseInt(defaultContributionId)
      if (!isNaN(contributionId)) {
        checkContributionStatus({ contributionId })
        setIsReady(true)
      }
    }
  }, [isReady, activeAccount, contract, activeSigner, api, router.query, trigger])

  if (!api) return null

  return (
    <>
      <div tw="flex grow flex-col space-y-4 max-w-[20rem]">
        <h2 tw="text-center font-mono text-sky-400">Demo Smart Contract</h2>

        {/* Update Greeting */}
        <Card variant="outline" p={4} bgColor="whiteAlpha.100">
          <form
            onSubmit={handleSubmit((inputs) => {
              router.push(`/check?contributionId=${encodeURIComponent(inputs.contributionId)}`)
              return checkContributionStatus(inputs)
            })}
          >
            <Stack direction="row" spacing={2} align="end">
              <FormControl>
                <FormLabel>Check Contribution</FormLabel>
                <Input
                  isDisabled={updateIsLoading}
                  placeholder="GitHub Issue Number"
                  defaultValue={defaultContributionId}
                  {...register('contributionId', {
                    required: true,
                    validate: (value) => !isNaN(value),
                  })}
                />
              </FormControl>
              <Button
                type="submit"
                mt={4}
                colorScheme="blue"
                isLoading={updateIsLoading}
                isDisabled={!formState.isValid}
              >
                Check
              </Button>
            </Stack>
          </form>
        </Card>

        {/* Contract Address */}
        <p tw="text-center font-mono text-xs text-sky-600">
          {contract ? contractAddress : 'Loading…'}
        </p>
      </div>
    </>
  )
}
