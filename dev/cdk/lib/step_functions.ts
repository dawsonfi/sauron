import { Construct } from 'constructs'
import { Function } from 'aws-cdk-lib/aws-lambda'
import { StateMachine, TaskInput, Choice, Condition, Succeed, Fail } from 'aws-cdk-lib/aws-stepfunctions'
import { LambdaInvoke } from 'aws-cdk-lib/aws-stepfunctions-tasks'

export class SauronStateMachine extends Construct {
    constructor(scope: Construct, id: string, lambdaFunction: Function) {
        super(scope, id)

        const success = new Succeed(this, 'DeuBom')
        const fail = new Fail(this, 'DeuRuim')

        new StateMachine(this, 'SauronStateMachine', {
            stateMachineName: 'SauronMachine',
            definition: new LambdaInvoke(this, 'Invoke', {
                lambdaFunction: lambdaFunction,
                outputPath: '$.Payload',
                payload: TaskInput.fromJsonPathAt('$'),
            }).next(
                new Choice(this, 'Processor')
                    .when(Condition.numberEquals('$.statusCode', 200), success)
                    .when(Condition.numberEquals('$.statusCode', 404), fail),
            ),
        })
    }
}
