pipeline {
    agent any
    options { 
        buildDiscarder logRotator(artifactDaysToKeepStr: '', artifactNumToKeepStr: '', daysToKeepStr: '', numToKeepStr: '5')
        disableConcurrentBuilds()
        parallelsAlwaysFailFast()
    }
    stages {
        stage('Run tests') {
            steps {
                echo """
                Job name: ${JOB_NAME}
                Git branch: ${GIT_BRANCH}
                Git commit: ${GIT_COMMIT}
                Git prev commit: ${GIT_PREVIOUS_COMMIT}
                Git prev successful commit: ${GIT_PREVIOUS_SUCCESSFUL_COMMIT}
                Git URL: ${GIT_URL}
                """
                script {
                    def params = [
                        [
                            $class: 'StringParameterValue',
                            name: 'ton_client_rs_branch',
                            value: "${GIT_BRANCH}"
                        ],
                        [
                            $class: 'StringParameterValue',
                            name: 'ton_client_rs_commit',
                            value: "${GIT_COMMIT}"
                        ]
                    ] 

                    build job: "Integration/sdk-intg-test/feature-add-pipeline", parameters: params
                }
            }
        }
    }
}