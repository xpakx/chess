package io.github.xpakx.chess.game;

import io.github.xpakx.chess.clients.MovePublisher;
import io.github.xpakx.chess.game.dto.EngineMoveEvent;
import lombok.RequiredArgsConstructor;
import org.springframework.amqp.AmqpRejectAndDontRequeueException;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class EngineEventHandler {
    private final GameService service;

    @RabbitListener(queues = "${amqp.queue.engine}")
    void handleMove(final EngineMoveEvent event) {
        try {
            service.doMakeMove(event);
        } catch (final Exception e) {
            throw new AmqpRejectAndDontRequeueException(e);
        }
    }
}
