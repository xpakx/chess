package io.github.xpakx.chess.settings;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.module.paramnames.ParameterNamesModule;
import org.springframework.amqp.core.*;
import org.springframework.amqp.rabbit.annotation.RabbitListenerConfigurer;
import org.springframework.amqp.support.converter.Jackson2JsonMessageConverter;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.messaging.converter.MappingJackson2MessageConverter;
import org.springframework.messaging.handler.annotation.support.DefaultMessageHandlerMethodFactory;
import org.springframework.messaging.handler.annotation.support.MessageHandlerMethodFactory;

@Configuration
public class AMQPConfig {
    private final String movesTopic;
    private final String gamesTopic;
    private final String updatesTopic;

    public AMQPConfig(
            @Value("${amqp.exchange.moves}") final String movesTopic,
            @Value("${amqp.exchange.games}") final String gamesTopic,
            @Value("${amqp.exchange.updates}") final String updatesTopic
    ) {
        this.movesTopic = movesTopic;
        this.gamesTopic = gamesTopic;
        this.updatesTopic= updatesTopic;
    }

    @Bean
    public TopicExchange movesTopicExchange() {
        return ExchangeBuilder
                .topicExchange(movesTopic)
                .durable(true)
                .build();
    }

    @Bean
    public TopicExchange gamesTopicExchange() {
        return ExchangeBuilder
                .topicExchange(gamesTopic)
                .durable(true)
                .build();
    }

    @Bean
    public TopicExchange updatesTopicExchange() {
        return ExchangeBuilder
                .topicExchange(updatesTopic)
                .durable(true)
                .build();
    }

    @Bean
    public Jackson2JsonMessageConverter producerJackson2MessageConverter() {
        return new Jackson2JsonMessageConverter();
    }

    @Bean
    public TopicExchange engineTopicExchange(@Value("${amqp.exchange.engine}") final String exchangeName) {
        return ExchangeBuilder.topicExchange(exchangeName).durable(true).build();
    }

    @Bean
    public Queue aiMovesQueue(@Value("${amqp.queue.engine}") final String queueName) {
        return QueueBuilder.durable(queueName).build();
    }

    @Bean
    public Binding aiMovesBinding(final Queue aiMovesQueue, final TopicExchange engineTopicExchange) {
        return BindingBuilder.bind(aiMovesQueue)
                .to(engineTopicExchange)
                .with("engine");
    }

    @Bean
    public MessageHandlerMethodFactory messageHandlerMethodFactory() {
        DefaultMessageHandlerMethodFactory factory = new DefaultMessageHandlerMethodFactory();
        final MappingJackson2MessageConverter jsonConverter = new MappingJackson2MessageConverter();
        jsonConverter.getObjectMapper().registerModule(
                new ParameterNamesModule(JsonCreator.Mode.PROPERTIES));
        factory.setMessageConverter(jsonConverter);
        return factory;
    }

    @Bean
    public RabbitListenerConfigurer rabbitListenerConfigurer(
            final MessageHandlerMethodFactory messageHandlerMethodFactory) {
        return (c) -> c.setMessageHandlerMethodFactory(messageHandlerMethodFactory);
    }


    @Bean
    public TopicExchange stateTopicExchange(@Value("${amqp.exchange.state}") final String exchangeName) {
        return ExchangeBuilder.topicExchange(exchangeName).durable(true).build();
    }

    @Bean
    public Queue stateQueue(@Value("${amqp.queue.state}") final String queueName) {
        return QueueBuilder.durable(queueName).build();
    }

    @Bean
    public Binding stateBinding(final Queue stateQueue, final TopicExchange stateTopicExchange) {
        return BindingBuilder.bind(stateQueue)
                .to(stateTopicExchange)
                .with("state");
    }
}
