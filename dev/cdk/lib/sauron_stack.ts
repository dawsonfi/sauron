import { Stack, StackProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { SauronLambda } from './lambda'
import { CfnQueryDefinition } from 'aws-cdk-lib/aws-logs'

export class SauronStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props)

        const sauronLambda = new SauronLambda(this, 'SauronLambda')
        new CfnQueryDefinition(this, 'ListLogsQuery', {
            name: 'ListLogs',
            queryString: 'fields @timestamp, @message | sort @timestamp desc',
            logGroupNames: [sauronLambda.sauronLambdaHandler.logGroup.logGroupName],
        })
    }
}
