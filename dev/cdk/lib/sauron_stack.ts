import { Stack, StackProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { SauronLambda } from './lambda'
import { SauronStateMachine } from './step_functions'

export class SauronStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props)

        const sauronLambda = new SauronLambda(this, 'SauronLambda')
        new SauronStateMachine(this, 'SauronStateMachine', sauronLambda.sauronLambdaHandler)
    }
}
