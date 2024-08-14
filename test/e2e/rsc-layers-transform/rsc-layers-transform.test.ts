import { nextTestSetup } from 'e2e-utils'

// TODO: support react-server condition for instrumentation hook in turbopack
;(process.env.TURBOPACK ? describe.skip : describe)(
  'rsc layers transform',
  () => {
    const { next } = nextTestSetup({
      files: __dirname,
    })

    it('should render installed react-server condition for middleware', async () => {
      const json = await next.fetch('/middleware').then((res) => res.json())

      expect(json).toEqual({
        textValue: 'text-value',
        clientReference: 'Symbol(react.client.reference)',
      })
    })

    it('should call instrumentation hook without errors', async () => {
      const output = next.cliOutput
      expect(output).toContain('instrumentation:register')
      expect(output).toContain('instrumentation:text:text-value')
    })
  }
)
